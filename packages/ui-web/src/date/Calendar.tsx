import { ChevronLeft, ChevronRight } from 'lucide-react';
import { cn } from '../lib/cn';
import { IconTooltipButton } from '../button/IconTooltipButton';
import { useUiCopy } from '../copy/useUiCopy';
import { useCalendar } from './useCalendar';

export type CalendarRange = { start: string | null; end: string | null };

type CalendarProps = {
  selected?: string | null;
  range?: CalendarRange;
  onSelectDay: (iso: string) => void;
  minDay?: string;
  maxDay?: string;
  today?: string;
  className?: string;
};

export const Calendar = ({ selected, range, onSelectDay, minDay, maxDay, today, className }: CalendarProps) => {
  const { date } = useUiCopy();
  const view = useCalendar({ selected, range, minDay, maxDay, today, onSelectDay });
  return (
    <div className={cn('w-full', className)}>
      <div className="mb-2 flex items-center justify-between gap-2">
        <IconTooltipButton icon={<ChevronLeft />} label={date.previousMonth} onClick={view.previous} />
        <button
          type="button"
          onClick={view.goToToday}
          title={date.today}
          className="rounded-md px-2 py-1 text-body font-semibold capitalize text-foreground transition-colors duration-200 ease-in-out hover:bg-muted focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
        >
          {view.label}
        </button>
        <IconTooltipButton icon={<ChevronRight />} label={date.nextMonth} onClick={view.next} />
      </div>
      <div className="grid grid-cols-7 gap-y-0.5 text-center" role="group" aria-label={view.label}>
        {view.weekdays.map((weekday) => (
          <span key={weekday} aria-hidden="true" className="py-1 text-label capitalize text-muted-foreground">
            {weekday}
          </span>
        ))}
        {view.cells.map((cell) => (
          <button
            key={cell.iso}
            type="button"
            disabled={cell.disabled}
            aria-pressed={cell.pressed}
            aria-label={cell.label}
            aria-current={cell.current ? 'date' : undefined}
            onClick={cell.select}
            className={cell.className}
          >
            {cell.day}
          </button>
        ))}
      </div>
    </div>
  );
};
