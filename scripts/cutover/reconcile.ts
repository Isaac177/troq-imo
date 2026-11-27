export type SnapshotRow = { table: string; city: string; key: string[] };
export type Finding = { kind: string; table: string; key: string[]; cities: string[] };
type Stored = SnapshotRow & { occurrences: number };

const identity = (row: SnapshotRow) => JSON.stringify([row.table, row.key]);
const scopedIdentity = (row: SnapshotRow) => JSON.stringify([row.table, row.city, row.key]);

export function parseRow(text: string, source: string, line: number): SnapshotRow {
  const fail = () => new Error(`${source}:${line}: invalid snapshot row`);
  let value: unknown;
  try { value = JSON.parse(text); } catch { throw fail(); }
  if (!value || typeof value !== 'object' || Array.isArray(value)) throw fail();
  const row = value as Record<string, unknown>;
  if (typeof row.table !== 'string' || !row.table.trim() || typeof row.city !== 'string' || !row.city.trim() || !Array.isArray(row.key) || row.key.length === 0 || !row.key.every((part) => typeof part === 'string' && part.length > 0)) throw fail();
  return { table: row.table, city: row.city, key: row.key as string[] };
}

export function reconcile(sourceRows: SnapshotRow[], targetRows: SnapshotRow[]) {
  const source = index(sourceRows);
  const target = index(targetRows);
  const findings: Finding[] = [];
  const totals = new Map<string, { table: string; city: string; source: number; target: number }>();
  const addTotal = (row: SnapshotRow, side: 'source' | 'target') => {
    const key = JSON.stringify([row.table, row.city]);
    const total = totals.get(key) ?? { table: row.table, city: row.city, source: 0, target: 0 };
    total[side] += 1;
    totals.set(key, total);
  };
  sourceRows.forEach((row) => addTotal(row, 'source'));
  targetRows.forEach((row) => addTotal(row, 'target'));
  for (const [key, row] of source.scoped) {
    if (row.occurrences > 1) findings.push({ kind: 'duplicate_source', table: row.table, key: row.key, cities: [row.city] });
    if (!target.scoped.has(key)) findings.push({ kind: 'missing_target', table: row.table, key: row.key, cities: [row.city] });
  }
  for (const [key, row] of target.scoped) {
    if (row.occurrences > 1) findings.push({ kind: 'duplicate_target', table: row.table, key: row.key, cities: [row.city] });
    if (!source.scoped.has(key)) findings.push({ kind: 'extra_target', table: row.table, key: row.key, cities: [row.city] });
  }
  for (const group of source.global.values()) {
    const cities = [...group.cities].sort();
    if (cities.length > 1) findings.push({ kind: 'cross_city_key', table: group.row.table, key: group.row.key, cities });
  }
  findings.sort((a, b) => JSON.stringify([a.table, a.key, a.kind, a.cities]).localeCompare(JSON.stringify([b.table, b.key, b.kind, b.cities])));
  return { totals: [...totals.values()].sort((a, b) => a.table.localeCompare(b.table) || a.city.localeCompare(b.city)), findings };
}

function index(rows: SnapshotRow[]) {
  const scoped = new Map<string, Stored>();
  const global = new Map<string, { row: SnapshotRow; cities: Set<string> }>();
  for (const row of rows) {
    const scopedKey = scopedIdentity(row);
    const existing = scoped.get(scopedKey);
    if (existing) existing.occurrences += 1;
    else scoped.set(scopedKey, { ...row, occurrences: 1 });
    const globalKey = identity(row);
    const group = global.get(globalKey) ?? { row, cities: new Set<string>() };
    group.cities.add(row.city);
    global.set(globalKey, group);
  }
  return { scoped, global };
}
