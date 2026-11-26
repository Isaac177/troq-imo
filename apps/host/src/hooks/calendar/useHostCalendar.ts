import { useEffect, useMemo, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { addDays, formatCdf, formatIsoDay, parseIsoDay, toIsoDay } from '@mungini/utils';
import { hostApi } from '@/services/host/host.api';
import { getCalendar } from '@/services/calendar/calendar.api';
import type { CalendarData, PropertyOption } from '@/types/calendar/calendar';
import { useCalendarViewMode } from '@/hooks/calendar/useCalendarViewMode';

const PAGE_SIZE = 10;
const SPAN_DAYS = 14;
const MAX_NIGHTS = 93;
export const RESTRICTION_KINDS = ['MIN_NIGHTS', 'MAX_NIGHTS', 'ADVANCE_NOTICE_DAYS', 'MAX_HORIZON_DAYS', 'TURNOVER_BUFFER_NIGHTS', 'CLOSED_TO_ARRIVAL', 'CLOSED_TO_DEPARTURE'];
const today = () => new Date().toISOString().slice(0, 10);

export const useHostCalendar = () => {
  const { t, i18n } = useTranslation('hostCalendar');
  const { t: th } = useTranslation('host');
  const translate = t as unknown as (key: string) => string;
  const translateHost = th as unknown as (key: string) => string;
  const { session, client, can } = useSession();
  const { mode, setMode } = useCalendarViewMode();
  const hosts = session?.hosts ?? [];
  const [hostId, setHostId] = useState(hosts[0]?.hostId ?? '');
  const [properties, setProperties] = useState<PropertyOption[]>([]);
  const [propertyId, setPropertyId] = useState('');
  const [range, setRange] = useState<{ start: string | null; end: string | null }>({ start: today(), end: addDays(today(), SPAN_DAYS) });
  const [page, setPage] = useState(1);
  const [calendar, setCalendar] = useState<CalendarData | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [selected, setSelected] = useState<string | null>(null);
  const [reloadToken, setReloadToken] = useState(0);
  const french = i18n.language.startsWith('fr');
  const span = range.start && range.end ? (Date.parse(range.end) - Date.parse(range.start)) / 86400000 : 0;
  const validRange = span > 0 && span <= MAX_NIGHTS;

  const shiftMonth = (offset: number) => {
    if (!range.start) return;
    const parts = parseIsoDay(range.start);
    if (!parts) return;
    const target = new Date(Date.UTC(parts.year, parts.month - 1 + offset, 1));
    const lastDay = new Date(Date.UTC(target.getUTCFullYear(), target.getUTCMonth() + 1, 0)).getUTCDate();
    const start = toIsoDay({ year: target.getUTCFullYear(), month: target.getUTCMonth() + 1, day: Math.min(parts.day, lastDay) });
    const currentSpan = range.end ? Math.round((Date.parse(range.end) - Date.parse(range.start)) / 86400000) : SPAN_DAYS;
    const nights = Number.isFinite(currentSpan) ? Math.min(Math.max(currentSpan, 1), MAX_NIGHTS) : SPAN_DAYS;
    setRange({ start, end: addDays(start, nights) });
  };

  useEffect(() => {
    if (!hosts.some((host) => host.hostId === hostId)) setHostId(hosts[0]?.hostId ?? '');
  }, [hosts, hostId]);

  useEffect(() => {
    if (!hostId) {
      setProperties([]);
      setPropertyId('');
      return;
    }
    let active = true;
    void hostApi(client).properties(hostId, 1, 100, '').then((result) => {
      if (!active || result.error) return;
      const options = (result.data?.items ?? []).map((property) => ({
        value: property.id,
        label: (french ? property.nameFr : property.nameEn) || property.nameEn || property.nameFr,
      }));
      setProperties(options);
      setPropertyId((current) => (options.some((option) => option.value === current) ? current : options[0]?.value ?? ''));
    });
    return () => {
      active = false;
    };
  }, [client, hostId, french]);

  useEffect(() => {
    setPage(1);
  }, [propertyId, range.start, range.end]);

  useEffect(() => {
    if (!propertyId || !range.start || !range.end || !validRange) {
      setCalendar(null);
      return;
    }
    let active = true;
    setLoading(true);
    setError(null);
    void getCalendar(client, propertyId, range.start, range.end, page).then((result) => {
      if (!active) return;
      setLoading(false);
      if (result.error || !result.data) {
        setError(translate('error'));
        setCalendar(null);
        return;
      }
      setCalendar(result.data);
    });
    return () => {
      active = false;
    };
  }, [client, propertyId, range.start, range.end, validRange, page, reloadToken, translate]);

  const unitName = (unitTypeId: string) => {
    const unit = calendar?.units.find((item) => item.unitTypeId === unitTypeId);
    if (!unit) return '';
    return (french ? unit.nameFr : unit.nameEn) || unit.nameEn || unit.nameFr;
  };

  const strips = useMemo(
    () => (calendar?.units ?? []).map((unit) => ({
      unitTypeId: unit.unitTypeId,
      name: (french ? unit.nameFr : unit.nameEn) || unit.nameEn || unit.nameFr,
      days: unit.days.map((day) => ({
        date: day.date,
        label: formatIsoDay(day.date),
        allocated: day.allocated,
        capacity: day.capacity,
        full: day.allocated >= day.capacity,
        price: day.price ? formatCdf(day.price, i18n.language) : '',
      })),
    })),
    [calendar, french, i18n.language],
  );

  const unitOptions = useMemo(() => (calendar?.units ?? []).map((unit) => ({ value: unit.unitTypeId, label: (french ? unit.nameFr : unit.nameEn) || unit.nameEn || unit.nameFr })), [calendar, french]);
  const unitPages = Math.max(1, Math.ceil((calendar?.totalUnits ?? 0) / PAGE_SIZE));

  return {
    hosts,
    hasHost: hosts.length > 0,
    hostId,
    setHostId,
    properties,
    propertyId,
    setPropertyId,
    range,
    setRange,
    mode,
    setMode,
    viewOptions: [
      { value: 'strip', label: translate('viewStrip') },
      { value: 'month', label: translate('viewMonth') },
    ],
    previousMonth: () => shiftMonth(-1),
    nextMonth: () => shiftMonth(1),
    validRange,
    page,
    setPage,
    previousPage: () => setPage((current) => Math.max(1, current - 1)),
    nextPage: () => setPage((current) => current + 1),
    unitPages,
    calendar,
    loading,
    error,
    strips,
    unitOptions,
    canWrite: can('inventory.write'),
    selected,
    setSelected,
    refresh: () => setReloadToken((value) => value + 1),
    unitName,
    reservations: (calendar?.reservations ?? []).map((item) => ({ id: item.id, reference: item.reference, guest: item.guestName ?? '', unit: unitName(item.unitTypeId), checkIn: formatIsoDay(item.checkIn), checkOut: formatIsoDay(item.checkOut), total: formatCdf(item.total, i18n.language), state: item.state })),
    blocks: (calendar?.blocks ?? []).map((item) => ({ id: item.id, reason: item.reason, unit: item.unitTypeId ? unitName(item.unitTypeId) : translate('allUnits'), start: formatIsoDay(item.startDate), end: formatIsoDay(item.endDate), version: item.version })),
    restrictions: (calendar?.restrictions ?? []).map((item) => ({ id: item.id, name: RESTRICTION_KINDS.includes(item.kind) ? translate(`restrictionKinds.${item.kind}`) : translate('restrictions'), value: item.value, start: item.startDate ? formatIsoDay(item.startDate) : '', end: item.endDate ? formatIsoDay(item.endDate) : '' })),
    labels: {
      title: translate('title'),
      subtitle: translate('subtitle'),
      property: translate('property'),
      chooseProperty: translate('chooseProperty'),
      range: translate('range'),
      view: translate('view'),
      viewStrip: translate('viewStrip'),
      viewMonth: translate('viewMonth'),
      invalidRange: translate('invalidRange'),
      previous: translate('previous'),
      next: translate('next'),
      loading: translate('loading'),
      error: translate('error'),
      units: translate('units'),
      unitsEmpty: translate('unitsEmpty'),
      capacity: translate('capacity'),
      allocated: translate('allocated'),
      open: translate('open'),
      full: translate('full'),
      reservations: translate('reservations'),
      noReservations: translate('noReservations'),
      guest: translate('guest'),
      blocks: translate('blocks'),
      noBlocks: translate('noBlocks'),
      blockDates: translate('blockDates'),
      removeBlock: translate('removeBlock'),
      restrictions: translate('restrictions'),
      noRestrictions: translate('noRestrictions'),
      noHost: translateHost('noHost'),
      noHostAction: translateHost('noHostAction'),
    },
  };
};
