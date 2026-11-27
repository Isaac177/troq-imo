import { createInterface } from 'node:readline';
import { createReadStream } from 'node:fs';
import { parseRow, reconcile, type SnapshotRow } from './reconcile';

async function load(path: string): Promise<SnapshotRow[]> {
  const rows: SnapshotRow[] = [];
  let line = 0;
  for await (const text of createInterface({ input: createReadStream(path), crlfDelay: Infinity })) {
    line += 1;
    if (text.trim()) rows.push(parseRow(text, path, line));
  }
  return rows;
}

async function main() {
  const [sourcePath, targetPath, ...extra] = process.argv.slice(2);
  if (!sourcePath || !targetPath || extra.length) throw new Error('Usage: bun scripts/cutover/reconcile-cli.ts source.jsonl target.jsonl');
  const [source, target] = await Promise.all([load(sourcePath), load(targetPath)]);
  if (!source.length) throw new Error('Source snapshot is empty; no reconciliation was performed.');
  const report = reconcile(source, target);
  process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
  if (report.findings.length) process.exitCode = 1;
}

main().catch((error: unknown) => {
  process.stderr.write(`${error instanceof Error ? error.message : String(error)}\n`);
  process.exitCode = 2;
});
