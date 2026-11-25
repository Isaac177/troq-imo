import { pad2, toIsoDay } from '@mungini/utils';

export type CalendarCell = { iso: string; day: number; inMonth: boolean };

export type MonthRef = { year: number; month: number };

export const shiftMonth = (ref: MonthRef, amount: number): MonthRef => {
  const index = ref.year * 12 + (ref.month - 1) + amount;
  return { year: Math.floor(index / 12), month: (index % 12) + 1 };
};

export const buildMonthWeeks = (ref: MonthRef): CalendarCell[][] => {
  const first = new Date(Date.UTC(ref.year, ref.month - 1, 1));
  const offset = (first.getUTCDay() + 6) % 7;
  const weeks: CalendarCell[][] = [];
  for (let week = 0; week < 6; week += 1) {
    const cells: CalendarCell[] = [];
    for (let weekday = 0; weekday < 7; weekday += 1) {
      const date = new Date(Date.UTC(ref.year, ref.month - 1, 1 - offset + week * 7 + weekday));
      cells.push({
        iso: toIsoDay({ year: date.getUTCFullYear(), month: date.getUTCMonth() + 1, day: date.getUTCDate() }),
        day: date.getUTCDate(),
        inMonth: date.getUTCMonth() === ref.month - 1,
      });
    }
    weeks.push(cells);
  }
  return weeks;
};

export const monthLabel = (ref: MonthRef, locale: string): string =>
  new Intl.DateTimeFormat(locale, { month: 'long', year: 'numeric', timeZone: 'UTC' }).format(new Date(Date.UTC(ref.year, ref.month - 1, 1)));

export const weekdayLabels = (locale: string): string[] => {
  const formatter = new Intl.DateTimeFormat(locale, { weekday: 'short', timeZone: 'UTC' });
  return Array.from({ length: 7 }, (_, index) => formatter.format(new Date(Date.UTC(2024, 0, 1 + index))));
};

export const localTodayIso = (): string => {
  const now = new Date();
  return `${String(now.getFullYear()).padStart(4, '0')}-${pad2(now.getMonth() + 1)}-${pad2(now.getDate())}`;
};
