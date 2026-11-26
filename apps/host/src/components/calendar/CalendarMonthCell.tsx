import { cn } from '@mungini/ui-web';
import type { CalendarMonthDay } from '@/hooks/calendar/useCalendarMonth';

type Props = {
  cell: CalendarMonthDay;
  fullLabel: string;
  onSelect: (date: string) => void;
};

export default function CalendarMonthCell({ cell, fullLabel, onSelect }: Props) {
  if (!cell.inRange) {
    return (
      <div title={cell.ariaLabel} className="min-w-0 rounded-md px-1 py-1">
        <span className="block text-micro tabular-nums text-muted-foreground/50">{cell.dayNumber}</span>
      </div>
    );
  }

  return (
    <button
      type="button"
      aria-label={cell.ariaLabel}
      aria-haspopup="dialog"
      onClick={() => onSelect(cell.date)}
      title={cell.ariaLabel}
      className={cn(
        'flex min-w-0 flex-col items-start gap-0.5 rounded-md px-1 py-1 text-left transition-colors duration-200 ease-in-out hover:bg-muted/60 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring',
        cell.isToday && 'bg-muted/60',
      )}
    >
      <span className="block text-label font-medium tabular-nums text-foreground">{cell.dayNumber}</span>
      <span className="block w-full truncate text-body tabular-nums text-foreground">{cell.allocated}/{cell.capacity}</span>
      <span className="block w-full truncate text-micro tabular-nums text-muted-foreground">{cell.priceLabel}</span>
      {cell.full ? (
        <span className="inline-flex min-w-0 items-center gap-1 text-micro text-foreground">
          <span className="size-1.5 shrink-0 rounded-full bg-primary" aria-hidden="true" />
          <span className="truncate">{fullLabel}</span>
        </span>
      ) : null}
    </button>
  );
}
