import { useEffect, useMemo, useState } from 'react';
import { formatIsoDay, parseIsoDay } from '@mungini/utils';
import { useUiCopy } from '../copy/useUiCopy';
import { cn } from '../lib/cn';
import type { CalendarRange } from './Calendar';
import { buildMonthWeeks, localTodayIso, monthLabel, shiftMonth, weekdayLabels, type MonthRef } from './calendarGrid';

type Options = {
  selected?: string | null;
  range?: CalendarRange;
  minDay?: string;
  maxDay?: string;
  today?: string;
  onSelectDay: (day: string) => void;
};

export const useCalendar = ({ selected, range, minDay, maxDay, today, onSelectDay }: Options) => {
  const { locale } = useUiCopy();
  const todayIso = today ?? localTodayIso();
  const focusDay = selected ?? range?.start;
  const initial = parseIsoDay(focusDay) ?? parseIsoDay(todayIso);
  const [view, setView] = useState<MonthRef>(() => ({ year: initial?.year ?? new Date().getFullYear(), month: initial?.month ?? 1 }));
  const weeks = useMemo(() => buildMonthWeeks(view), [view]);
  const weekdays = useMemo(() => weekdayLabels(locale), [locale]);

  useEffect(() => {
    const parts = parseIsoDay(focusDay);
    if (parts) setView({ year: parts.year, month: parts.month });
  }, [focusDay]);

  const cells = weeks.flat().map((cell) => {
    const disabled = (minDay !== undefined && cell.iso < minDay) || (maxDay !== undefined && cell.iso > maxDay);
    const isStart = cell.iso === (range ? range.start : selected);
    const isEnd = range !== undefined && cell.iso === range.end;
    const inRange = range !== undefined && range.start !== null && range.end !== null && cell.iso > range.start && cell.iso < range.end;
    const edge = isStart || isEnd;
    return {
      ...cell,
      disabled,
      pressed: edge,
      label: formatIsoDay(cell.iso),
      current: cell.iso === todayIso,
      select: () => onSelectDay(cell.iso),
      className: cn(
        'h-8 text-body tabular-nums transition-colors duration-200 ease-in-out focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-30',
        edge ? 'rounded-md bg-primary font-medium text-primary-foreground' : inRange ? 'bg-accent text-foreground' : 'rounded-md text-foreground hover:bg-muted',
        !cell.inMonth && !edge && 'text-muted-foreground',
        cell.iso === todayIso && !edge && 'font-semibold text-tone-identity',
      ),
    };
  });

  return {
    cells,
    weekdays,
    label: monthLabel(view, locale),
    previous: () => setView((current) => shiftMonth(current, -1)),
    next: () => setView((current) => shiftMonth(current, 1)),
    goToToday: () => {
      const parts = parseIsoDay(todayIso);
      if (parts) setView({ year: parts.year, month: parts.month });
    },
  };
};
