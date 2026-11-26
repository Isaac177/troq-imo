type Day = { date: string; label: string; allocated: number; capacity: number; full: boolean; price: string };

type Props = {
  name: string;
  days: Day[];
  labels: { open: string; full: string };
};

export default function UnitStrip({ name, days, labels }: Props) {
  return (
    <div className="flex flex-col gap-2">
      <h3 className="text-body font-semibold text-foreground">{name}</h3>
      <div className="flex gap-2 overflow-x-auto pb-1">
        {days.map((day) => (
          <div key={day.date} className="flex min-w-[84px] shrink-0 flex-col gap-0.5 rounded-md bg-muted/40 px-2 py-1.5 text-label">
            <span className="tabular-nums text-muted-foreground">{day.label}</span>
            <span className="text-body font-medium tabular-nums text-foreground">{day.allocated}/{day.capacity}</span>
            <span className="truncate tabular-nums text-muted-foreground">{day.price}</span>
            <span className={day.full ? 'text-muted-foreground' : 'text-foreground'}>{day.full ? labels.full : labels.open}</span>
          </div>
        ))}
      </div>
    </div>
  );
}
