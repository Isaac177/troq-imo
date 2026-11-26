type Row = { id: string; name: string; value: number | null | undefined; start: string; end: string };

type Props = { rows: Row[]; empty: string };

export default function CalendarRestrictionList({ rows, empty }: Props) {
  if (rows.length === 0) return <p className="text-body text-muted-foreground">{empty}</p>;
  return (
    <ul className="divide-y divide-border">
      {rows.map((row) => (
        <li key={row.id} className="flex flex-wrap items-center justify-between gap-2 py-2 text-body">
          <span className="min-w-0">
            <span className="block truncate font-medium text-foreground">{row.name}{row.value === null || row.value === undefined ? '' : ` · ${row.value}`}</span>
            <span className="block truncate text-label tabular-nums text-muted-foreground">{row.start || '—'} – {row.end || '—'}</span>
          </span>
        </li>
      ))}
    </ul>
  );
}
