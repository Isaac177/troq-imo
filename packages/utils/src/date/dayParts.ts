export type DayParts = { year: number; month: number; day: number };

const DAY_PATTERN = /^(\d{4})-(\d{2})-(\d{2})$/;

export const pad2 = (value: number): string => String(value).padStart(2, '0');

export const parseIsoDay = (value: string | null | undefined): DayParts | null => {
  const match = value ? DAY_PATTERN.exec(value) : null;
  if (!match) return null;
  const parts = { year: Number(match[1]), month: Number(match[2]), day: Number(match[3]) };
  const probe = new Date(Date.UTC(parts.year, parts.month - 1, parts.day));
  const valid =
    probe.getUTCFullYear() === parts.year && probe.getUTCMonth() === parts.month - 1 && probe.getUTCDate() === parts.day;
  return valid ? parts : null;
};

export const toIsoDay = (parts: DayParts): string => `${String(parts.year).padStart(4, '0')}-${pad2(parts.month)}-${pad2(parts.day)}`;

export const addDays = (value: string, amount: number): string => {
  const parts = parseIsoDay(value);
  if (!parts) return value;
  const next = new Date(Date.UTC(parts.year, parts.month - 1, parts.day + amount));
  return toIsoDay({ year: next.getUTCFullYear(), month: next.getUTCMonth() + 1, day: next.getUTCDate() });
};

export const compareIsoDay = (left: string, right: string): number => (left < right ? -1 : left > right ? 1 : 0);

export const parseDisplayDay = (text: string): string | null => {
  const match = /^(\d{1,2})\/(\d{1,2})\/(\d{4})$/.exec(text.trim());
  if (!match) return null;
  const parts = { year: Number(match[3]), month: Number(match[2]), day: Number(match[1]) };
  const iso = toIsoDay(parts);
  return parseIsoDay(iso) ? iso : null;
};
