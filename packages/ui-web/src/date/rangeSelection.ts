import type { CalendarRange } from './Calendar';

export const nextRange = (current: CalendarRange, day: string): { range: CalendarRange; complete: boolean } => {
  if (current.start === null || current.end !== null || day < current.start) {
    return { range: { start: day, end: null }, complete: false };
  }
  return { range: { start: current.start, end: day }, complete: true };
};
