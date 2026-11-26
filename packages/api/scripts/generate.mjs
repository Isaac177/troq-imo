import { readdir, rename, stat, writeFile } from 'node:fs/promises';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import openapiTS, { astToString } from 'openapi-typescript';

const packageRoot = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const backendRoot = resolve(packageRoot, '../../backend');
const binary = resolve(process.env.MUNGINI_API_BIN || join(backendRoot, 'target/debug/mungini-api'));
const relevant = (path) => path.endsWith('.rs') || path.endsWith('.toml') || path.endsWith('.sql') || path.endsWith('Cargo.lock');

const newestSource = async (path) => {
  const entries = await readdir(path, { withFileTypes: true });
  const dates = await Promise.all(entries.filter((entry) => entry.name !== 'target' && !entry.name.startsWith('.')).map(async (entry) => {
    const current = join(path, entry.name);
    if (entry.isDirectory()) return newestSource(current);
    return entry.isFile() && relevant(current) ? (await stat(current)).mtimeMs : 0;
  }));
  return Math.max(0, ...dates);
};

const ensureCurrent = async (modified) => {
  if (await newestSource(backendRoot) > modified) {
    throw new Error('The API binary is older than backend source. Wait for the owner to rebuild it before generating the client.');
  }
};

const run = async () => {
  const modified = (await stat(binary)).mtimeMs;
  await ensureCurrent(modified);
  const process = Bun.spawn([binary, 'openapi'], { stdout: 'pipe', stderr: 'pipe' });
  const [output, errors, status] = await Promise.all([
    new Response(process.stdout).text(),
    new Response(process.stderr).text(),
    process.exited,
  ]);
  if (status !== 0) throw new Error(`API OpenAPI generation failed: ${errors.trim()}`);
  const document = JSON.parse(output);
  if (!document.openapi || !document.paths || Object.keys(document.paths).length === 0) {
    throw new Error('API returned an invalid or empty OpenAPI document.');
  }
  const nodes = await openapiTS(document);
  const types = astToString(nodes, { formatOptions: { removeComments: true } });
  if (!types.includes('export interface paths')) throw new Error('Generated API types do not contain paths.');
  await ensureCurrent(modified);
  const specPath = join(packageRoot, 'openapi.json');
  const typePath = join(packageRoot, 'generated.ts');
  const indexPath = join(packageRoot, 'index.ts');
  const index = "import createClient from 'openapi-fetch';\nimport type { ClientOptions } from 'openapi-fetch';\nimport type { paths } from './generated';\n\nexport type { paths, components, operations } from './generated';\nexport const createApiClient = (options: ClientOptions) => createClient<paths>(options);\n";
  await Promise.all([
    writeFile(`${specPath}.tmp`, `${JSON.stringify(document, null, 2)}\n`),
    writeFile(`${typePath}.tmp`, types),
    writeFile(`${indexPath}.tmp`, index),
  ]);
  await Promise.all([
    rename(`${specPath}.tmp`, specPath),
    rename(`${typePath}.tmp`, typePath),
    rename(`${indexPath}.tmp`, indexPath),
  ]);
};

run().catch((error) => {
  process.stderr.write(`${error.message}\n`);
  process.exitCode = 1;
});
