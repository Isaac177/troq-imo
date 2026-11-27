import { describe, expect, test } from 'bun:test';
import { parseRow, reconcile } from './reconcile';

describe('cutover reconciliation', () => {
  test('identifies cross-city collisions, missing and extra rows, and duplicate exports', () => {
    const row = (city: string, key: string) => ({ table: 'stay_reservations', city, key: [key] });
    const result = reconcile(
      [row('GOMA', 'shared'), row('KIN', 'shared'), row('GOMA', 'missing'), row('GOMA', 'missing')],
      [row('GOMA', 'shared'), row('KIN', 'shared'), row('KIN', 'extra')],
    );
    expect(result.findings.map((item) => item.kind).sort()).toEqual(['cross_city_key', 'duplicate_source', 'extra_target', 'missing_target']);
    expect(result.totals).toEqual([
      { table: 'stay_reservations', city: 'GOMA', source: 3, target: 1 },
      { table: 'stay_reservations', city: 'KIN', source: 1, target: 2 },
    ]);
  });

  test('rejects malformed keys and preserves composite key boundaries', () => {
    expect(() => parseRow('{"table":"t","city":"GOMA","key":[]}', 'snapshot', 3)).toThrow('snapshot:3');
    const rows = [
      { table: 't', city: 'GOMA', key: ['a:b', 'c'] },
      { table: 't', city: 'GOMA', key: ['a', 'b:c'] },
    ];
    expect(reconcile(rows, rows).findings).toEqual([]);
  });
});
