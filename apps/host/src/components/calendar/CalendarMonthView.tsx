import { ChevronLeft, ChevronRight } from 'lucide-react';
import { IconTooltipButton } from '@mungini/ui-web';
import CalendarMonthGrid from '@/components/calendar/CalendarMonthGrid';
import type { CalendarMonthState } from '@/hooks/calendar/useCalendarMonth';

type Props = {
  view: CalendarMonthState;
  onPreviousMonth: () => void;
  onNextMonth: () => void;
};

export default function CalendarMonthView({ view, onPreviousMonth, onNextMonth }: Props) {
  return (
    <div className="flex min-w-0 flex-col gap-3">
      <div className="flex min-w-0 items-center justify-between gap-2">
        <h2 className="min-w-0 truncate text-title font-semibold text-foreground">{view.monthLabel}</h2>
        <div className="flex shrink-0 items-center gap-1">
          <IconTooltipButton icon={<ChevronLeft />} label={view.labels.previousMonth} onClick={onPreviousMonth} />
          <IconTooltipButton icon={<ChevronRight />} label={view.labels.nextMonth} onClick={onNextMonth} />
        </div>
      </div>
      <CalendarMonthGrid weekdayLabels={view.weekdayLabels} weeks={view.weeks} fullLabel={view.labels.full} onSelectDay={view.openDay} />
      <p className="text-label text-muted-foreground">{view.labels.rangeHint}</p>
    </div>
  );
}
