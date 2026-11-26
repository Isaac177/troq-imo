type Row = { id: string; reference: string; guest: string; unit: string; checkIn: string; checkOut: string; total: string };

type Props = {
  rows: Row[];
  empty: string;
  guestLabel: string;
  onOpen?: (id: string) => void;
};

export default function CalendarReservationList({ rows, empty, guestLabel, onOpen }: Props) {
  if (rows.length === 0) return <p className="text-body text-muted-foreground">{empty}</p>;
  return (
    <ul className="divide-y divide-border">
      {rows.map((row) => (
        <li key={row.id}>
          <button
            type="button"
            onClick={() => onOpen?.(row.id)}
            className="flex w-full flex-wrap items-center justify-between gap-2 py-2 text-left text-body transition-colors duration-200 ease-in-out hover:bg-muted/40"
          >
            <span className="min-w-0">
              <span className="block truncate font-medium text-foreground">{row.reference}</span>
              <span className="block truncate text-label text-muted-foreground">{row.unit} · {row.guest || guestLabel}</span>
            </span>
            <span className="flex flex-col items-end text-label tabular-nums text-muted-foreground">
              <span>{row.checkIn} – {row.checkOut}</span>
              <span className="text-foreground">{row.total}</span>
            </span>
          </button>
        </li>
      ))}
    </ul>
  );
}
