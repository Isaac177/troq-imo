import CalendarMonthCell from '@/components/calendar/CalendarMonthCell';
import type { CalendarMonthDay } from '@/hooks/calendar/useCalendarMonth';

type Props = {
  weekdayLabels: string[];
  weeks: (CalendarMonthDay | null)[][];
  fullLabel: string;
  onSelectDay: (date: string) => void;
};

export default function CalendarMonthGrid({ weekdayLabels, weeks, fullLabel, onSelectDay }: Props) {
  return (
    <div className="min-w-0">
      <div className="grid grid-cols-7 gap-1">
        {weekdayLabels.map((label) => (
          <div key={label} className="min-w-0 truncate text-center text-micro text-muted-foreground">{label}</div>
        ))}
      </div>
      <div className="mt-1 flex flex-col gap-1">
        {weeks.map((week, weekIndex) => (
          <div key={weekIndex} className="grid grid-cols-7 gap-1">
            {week.map((cell, dayIndex) => (cell
              ? <CalendarMonthCell key={cell.date} cell={cell} fullLabel={fullLabel} onSelect={onSelectDay} />
              : <div key={`empty-${weekIndex}-${dayIndex}`} className="min-w-0" aria-hidden="true" />))}
          </div>
        ))}
      </div>
    </div>
  );
}
