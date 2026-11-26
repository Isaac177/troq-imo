import { Ban } from 'lucide-react';
import { Button } from '@mungini/ui-web';

type Row = { id: string; reason: string; unit: string; start: string; end: string; version: number };

type Props = {
  rows: Row[];
  empty: string;
  removeLabel?: string;
  onRemove?: (row: Row) => void;
};

export default function CalendarBlockList({ rows, empty, removeLabel, onRemove }: Props) {
  if (rows.length === 0) return <p className="text-body text-muted-foreground">{empty}</p>;
  return (
    <ul className="divide-y divide-border">
      {rows.map((row) => (
        <li key={row.id} className="flex flex-wrap items-center justify-between gap-2 py-2 text-body">
          <span className="min-w-0">
            <span className="block truncate font-medium text-foreground">{row.reason}</span>
            <span className="block truncate text-label text-muted-foreground">{row.unit} · {row.start} – {row.end}</span>
          </span>
          {onRemove && removeLabel ? <Button size="sm" variant="outline" icon={<Ban />} collapseLabel aria-label={removeLabel} title={removeLabel} onClick={() => onRemove(row)}>{removeLabel}</Button> : null}
        </li>
      ))}
    </ul>
  );
}
