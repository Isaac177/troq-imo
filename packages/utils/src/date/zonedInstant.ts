import { parseIsoDay } from './dayParts';
import { zonedWallClock } from './formatDay';

const offsetAt = (instantMs: number, timeZone: string): number => {
  const wall = zonedWallClock(new Date(instantMs), timeZone);
  const day = parseIsoDay(wall?.date);
  if (!wall || !day) return 0;
  const [hour = 0, minute = 0] = wall.time.split(':').map(Number);
  return Date.UTC(day.year, day.month - 1, day.day, hour, minute) - Math.floor(instantMs / 60000) * 60000;
};

export const wallClockToIso = (date: string, time: string, timeZone: string): string | null => {
  const day = parseIsoDay(date);
  const match = /^(\d{2}):(\d{2})$/.exec(time);
  if (!day || !match) return null;
  const hour = Number(match[1]);
  const minute = Number(match[2]);
  if (hour > 23 || minute > 59) return null;
  const wallMs = Date.UTC(day.year, day.month - 1, day.day, hour, minute);
  const first = wallMs - offsetAt(wallMs, timeZone);
  const second = wallMs - offsetAt(first, timeZone);
  const instant = new Date(second);
  const roundTrip = zonedWallClock(instant, timeZone);
  return roundTrip?.date === date && roundTrip.time === time ? instant.toISOString() : null;
};
