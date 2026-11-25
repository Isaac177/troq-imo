import { pad2, parseIsoDay } from './dayParts';

const toInstant = (value: string | Date | null | undefined): Date | null => {
  if (!value) return null;
  const date = value instanceof Date ? value : new Date(value);
  return Number.isNaN(date.getTime()) ? null : date;
};

const zonedParts = (date: Date, timeZone: string) => {
  const parts = new Intl.DateTimeFormat('en-GB', {
    timeZone,
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
    hourCycle: 'h23',
  }).formatToParts(date);
  const pick = (type: Intl.DateTimeFormatPartTypes): string => parts.find((part) => part.type === type)?.value ?? '';
  return { day: pick('day'), month: pick('month'), year: pick('year'), hour: pick('hour'), minute: pick('minute') };
};

export const formatIsoDay = (value: string | null | undefined): string => {
  const parts = parseIsoDay(value);
  return parts ? `${pad2(parts.day)}/${pad2(parts.month)}/${String(parts.year).padStart(4, '0')}` : '';
};

export const formatDay = (value: string | Date | null | undefined, timeZone: string): string => {
  if (typeof value === 'string' && parseIsoDay(value)) return formatIsoDay(value);
  const date = toInstant(value);
  if (!date) return '';
  const { day, month, year } = zonedParts(date, timeZone);
  return `${day}/${month}/${year}`;
};

export const formatDateTime = (value: string | Date | null | undefined, timeZone: string): string => {
  const date = toInstant(value);
  if (!date) return '';
  const { day, month, year, hour, minute } = zonedParts(date, timeZone);
  return `${day}/${month}/${year} ${hour}:${minute}`;
};

export const zonedWallClock = (value: string | Date, timeZone: string): { date: string; time: string } | null => {
  const instant = toInstant(value);
  if (!instant) return null;
  const { day, month, year, hour, minute } = zonedParts(instant, timeZone);
  return { date: `${year}-${month}-${day}`, time: `${hour}:${minute}` };
};
