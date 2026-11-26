import { useEffect, useMemo, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { formatCdf, formatIsoDay, parseIsoDay, toIsoDay, zonedWallClock } from '@mungini/utils';
import type { CalendarData } from '@/types/calendar/calendar';
import { RESTRICTION_KINDS } from '@/hooks/calendar/useHostCalendar';

type Range = { start: string | null; end: string | null };

export type CalendarMonthDay = {
  date: string;
  dayNumber: number;
  inRange: boolean;
  isToday: boolean;
  allocated: number;
  capacity: number;
  full: boolean;
  priceLabel: string;
  ariaLabel: string;
};

type DayTotal = { allocated: number; capacity: number; min: number | null; max: number | null };

const DAY_MS = 86400000;
const WEEK_REFERENCE = Date.UTC(2024, 0, 1);

export const useCalendarMonth = (calendar: CalendarData | null, range: Range, onOpenReservation: (id: string) => void) => {
  const { t, i18n } = useTranslation('hostCalendar');
  const translate = t as unknown as (key: string) => string;
  const [selectedDay, setSelectedDay] = useState<string | null>(null);
  const french = i18n.language.startsWith('fr');

  useEffect(() => {
    setSelectedDay(null);
  }, [range.start, range.end, calendar?.propertyId]);

  const unitNames = useMemo<Map<string, string>>(
    () => new Map((calendar?.units ?? []).map((unit) => [unit.unitTypeId, (french ? unit.nameFr : unit.nameEn) || unit.nameEn || unit.nameFr] as const)),
    [calendar, french],
  );

  const totals = useMemo(() => {
    const map = new Map<string, DayTotal>();
    for (const unit of calendar?.units ?? []) {
      for (const day of unit.days) {
        const total = map.get(day.date) ?? { allocated: 0, capacity: 0, min: null, max: null };
        total.allocated += day.allocated;
        total.capacity += day.capacity;
        const value = day.price === null || day.price === undefined || day.price === '' ? Number.NaN : Number(day.price);
        if (Number.isFinite(value)) {
          total.min = total.min === null ? value : Math.min(total.min, value);
          total.max = total.max === null ? value : Math.max(total.max, value);
        }
        map.set(day.date, total);
      }
    }
    return map;
  }, [calendar]);

  const month = useMemo(() => {
    const timezone = calendar?.timezone ?? 'UTC';
    const today = zonedWallClock(new Date(), timezone)?.date ?? '';
    const anchor = parseIsoDay(range.start ?? today);
    if (!anchor) return null;
    const firstWeekday = new Date(Date.UTC(anchor.year, anchor.month - 1, 1)).getUTCDay();
    const offset = (firstWeekday + 6) % 7;
    const daysInMonth = new Date(Date.UTC(anchor.year, anchor.month, 0)).getUTCDate();
    const cellCount = Math.ceil((offset + daysInMonth) / 7) * 7;
    const cells: (CalendarMonthDay | null)[] = Array.from({ length: cellCount }, (_, index) => {
      const dayNumber = index - offset + 1;
      if (dayNumber < 1 || dayNumber > daysInMonth) return null;
      const date = toIsoDay({ year: anchor.year, month: anchor.month, day: dayNumber });
      const total = totals.get(date);
      const inRange = Boolean(range.start && range.end && date >= range.start && date < range.end);
      const allocated = total?.allocated ?? 0;
      const capacity = total?.capacity ?? 0;
      const priceLabel = !total || total.min === null || total.max === null
        ? (inRange ? '—' : '')
        : total.min === total.max
          ? formatCdf(total.min, i18n.language, 0)
          : `${formatCdf(total.min, i18n.language, 0)} – ${formatCdf(total.max, i18n.language, 0)}`;
      const full = inRange && capacity > 0 && allocated >= capacity;
      const activity = inRange ? `${translate('allocated')} ${allocated} / ${translate('capacity')} ${capacity}, ${full ? translate('full') : translate('open')}` : translate('outsideRange');
      const price = inRange && priceLabel ? `, ${priceLabel}` : '';
      const ariaLabel = `${formatIsoDay(date)}${date === today ? `, ${translate('today')}` : ''}, ${activity}${price}`;
      return { date, dayNumber, inRange, isToday: date === today, allocated, capacity, full, priceLabel, ariaLabel };
    });
    const weeks = Array.from({ length: cellCount / 7 }, (_, index) => cells.slice(index * 7, index * 7 + 7));
    const label = new Intl.DateTimeFormat(i18n.language, { month: 'long', year: 'numeric', timeZone: 'UTC' })
      .format(new Date(Date.UTC(anchor.year, anchor.month - 1, 1)));
    const weekdayFormatter = new Intl.DateTimeFormat(i18n.language, { weekday: 'short', timeZone: 'UTC' });
    const weekdayLabels = Array.from({ length: 7 }, (_, index) => weekdayFormatter.format(new Date(WEEK_REFERENCE + index * DAY_MS)));
    return { label, weekdayLabels, weeks };
  }, [range.start, range.end, totals, i18n.language, translate, calendar?.timezone]);

  const dayTotal = selectedDay ? totals.get(selectedDay) : undefined;
  const selectedInRange = Boolean(selectedDay && range.start && range.end && selectedDay >= range.start && selectedDay < range.end);

  const reservations = useMemo(() => {
    if (!selectedDay) return [];
    return (calendar?.reservations ?? [])
      .filter((item) => item.checkIn <= selectedDay && item.checkOut > selectedDay)
      .map((item) => ({
        id: item.id,
        reference: item.reference,
        guest: item.guestName ?? '',
        unit: unitNames.get(item.unitTypeId) ?? '',
        checkIn: formatIsoDay(item.checkIn),
        checkOut: formatIsoDay(item.checkOut),
        total: formatCdf(item.total, i18n.language),
      }));
  }, [calendar, selectedDay, unitNames, i18n.language]);

  const blocks = useMemo(() => {
    if (!selectedDay) return [];
    return (calendar?.blocks ?? [])
      .filter((item) => item.startDate <= selectedDay && item.endDate > selectedDay)
      .map((item) => ({
        id: item.id,
        reason: item.reason,
        unit: item.unitTypeId ? unitNames.get(item.unitTypeId) ?? '' : translate('allUnits'),
        start: formatIsoDay(item.startDate),
        end: formatIsoDay(item.endDate),
        version: item.version,
      }));
  }, [calendar, selectedDay, unitNames, translate]);

  const restrictions = useMemo(() => {
    if (!selectedDay) return [];
    return (calendar?.restrictions ?? [])
      .filter((item) => (!item.startDate || item.startDate <= selectedDay) && (!item.endDate || item.endDate > selectedDay))
      .map((item) => ({
        id: item.id,
        name: RESTRICTION_KINDS.includes(item.kind) ? translate(`restrictionKinds.${item.kind}`) : translate('restrictions'),
        value: item.value,
        start: item.startDate ? formatIsoDay(item.startDate) : '',
        end: item.endDate ? formatIsoDay(item.endDate) : '',
      }));
  }, [calendar, selectedDay, translate]);

  const openDay = (date: string) => setSelectedDay(date);
  const closeDay = () => setSelectedDay(null);
  const openReservation = (id: string) => {
    setSelectedDay(null);
    onOpenReservation(id);
  };

  return {
    monthLabel: month?.label ?? '',
    weekdayLabels: month?.weekdayLabels ?? [],
    weeks: month?.weeks ?? [],
    selectedDay,
    openDay,
    closeDay,
    openReservation,
    sheetTitle: selectedDay ? formatIsoDay(selectedDay) : '',
    sheetDescription: selectedInRange ? `${translate('allocated')} ${dayTotal?.allocated ?? 0} / ${translate('capacity')} ${dayTotal?.capacity ?? 0}` : '',
    reservations,
    blocks,
    restrictions,
    labels: {
      previousMonth: translate('previousMonth'),
      nextMonth: translate('nextMonth'),
      rangeHint: translate('monthRangeHint'),
      reservations: translate('reservations'),
      noReservations: translate('noReservations'),
      blocks: translate('blocks'),
      noBlocks: translate('noBlocks'),
      restrictions: translate('restrictions'),
      noRestrictions: translate('noRestrictions'),
      full: translate('full'),
      guest: translate('guest'),
      close: translate('close'),
    },
  };
};

export type CalendarMonthState = ReturnType<typeof useCalendarMonth>;
