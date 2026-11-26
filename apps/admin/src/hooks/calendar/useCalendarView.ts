import { useEffect, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { useDebouncedValue } from '@mungini/ui-web';
import { formatCdf, formatIsoDay } from '@mungini/utils';
import { getCalendar, listProperties } from '@/services/calendar/calendar.api';
import type { CalendarData, PropertyPage } from '@/types/calendar/calendar';

const initialRange = () => {
  const start = new Date();
  const end = new Date(start.getTime() + 14 * 86400000);
  return { start: start.toISOString().slice(0, 10), end: end.toISOString().slice(0, 10) };
};

export const useCalendarView = () => {
  const { t, i18n } = useTranslation('admin');
  const { client, city } = useSession();
  const [search, setSearch] = useState('');
  const term = useDebouncedValue(search, 250);
  const [propertyPage, setPropertyPage] = useState(1);
  const [properties, setProperties] = useState<PropertyPage | null>(null);
  const [propertyId, setPropertyId] = useState('');
  const [range, setRange] = useState<{ start: string | null; end: string | null }>(initialRange);
  const [page, setPage] = useState(1);
  const [calendar, setCalendar] = useState<CalendarData | null>(null);
  const [propertyError, setPropertyError] = useState(false);
  const [calendarError, setCalendarError] = useState(false);
  const [loading, setLoading] = useState(false);
  const days = range.start && range.end ? (Date.parse(range.end) - Date.parse(range.start)) / 86400000 : 0;
  const validRange = days > 0 && days <= 93;

  useEffect(() => { setPropertyPage(1); setPropertyId(''); setCalendar(null); }, [city, term]);
  useEffect(() => {
    let active = true;
    void listProperties(client, term, propertyPage).then(({ data, error }) => {
      if (!active) return;
      if (error || !data) { setPropertyError(true); setProperties(null); return; }
      setPropertyError(false);
      setProperties(data.data);
    }).catch(() => { if (active) { setPropertyError(true); setProperties(null); } });
    return () => { active = false; };
  }, [client, city, term, propertyPage]);
  useEffect(() => { setPage(1); }, [propertyId, range.start, range.end, city]);
  useEffect(() => {
    if (!propertyId || !validRange || !range.start || !range.end) { setCalendar(null); setLoading(false); return; }
    let active = true;
    setLoading(true);
    setCalendarError(false);
    void getCalendar(client, propertyId, range.start, range.end, page).then(({ data, error }) => {
      if (!active) return;
      setLoading(false);
      if (error || !data) { setCalendarError(true); setCalendar(null); return; }
      setCalendar(data.data);
    }).catch(() => { if (active) { setLoading(false); setCalendarError(true); setCalendar(null); } });
    return () => { active = false; };
  }, [client, city, propertyId, range.start, range.end, validRange, page]);

  const french = i18n.language.startsWith('fr');
  const options = (properties?.items ?? []).map((item) => ({ value: item.id, label: (french ? item.nameFr || item.nameEn : item.nameEn || item.nameFr) + ` · ${item.hostName}` }));
  const propertyPages = Math.max(1, Math.ceil((properties?.total ?? 0) / 20));
  const unitPages = Math.max(1, Math.ceil((calendar?.totalUnits ?? 0) / 10));
  return {
    search, setSearch, propertyPage, setPropertyPage, propertyPages, propertyId,
    selectProperty: (value: string) => { setPropertyId(value); setPage(1); }, options,
    range, setRange, validRange, calendar, loading, error: propertyError || calendarError,
    page, setPage, unitPages, french,
    restrictionName: (kind: string) => {
      const keys: Record<string, 'calendarMinimumNights' | 'calendarMaximumNights' | 'calendarAdvanceNotice' | 'calendarMaximumHorizon' | 'calendarTurnoverBuffer' | 'calendarClosedArrival' | 'calendarClosedDeparture'> = { MIN_NIGHTS: 'calendarMinimumNights', MAX_NIGHTS: 'calendarMaximumNights', ADVANCE_NOTICE_DAYS: 'calendarAdvanceNotice', MAX_HORIZON_DAYS: 'calendarMaximumHorizon', TURNOVER_BUFFER_NIGHTS: 'calendarTurnoverBuffer', CLOSED_TO_ARRIVAL: 'calendarClosedArrival', CLOSED_TO_DEPARTURE: 'calendarClosedDeparture' };
      return keys[kind] ? t(keys[kind]) : t('calendarRestrictions');
    },
    day: formatIsoDay, money: (value: string) => formatCdf(value, i18n.language),
    labels: { title: t('calendarTitle'), subtitle: t('calendarSubtitle'), search: t('calendarSearch'), property: t('calendarProperty'), range: t('calendarRange'), invalidRange: t('calendarInvalidRange'), chooseProperty: t('calendarChooseProperty'), units: t('calendarUnits'), empty: t('calendarEmpty'), reservations: t('calendarReservations'), blocks: t('calendarBlocks'), restrictions: t('calendarRestrictions'), capacity: t('calendarCapacity'), allocated: t('calendarAllocated'), price: t('calendarPrice'), previous: t('calendarPrevious'), next: t('calendarNext'), loading: t('calendarLoading'), error: t('error'), noReservations: t('calendarNoReservations'), noBlocks: t('calendarNoBlocks'), noRestrictions: t('calendarNoRestrictions'), page: t('calendarPage') },
  };
};
