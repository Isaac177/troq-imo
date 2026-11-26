import { useEffect, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { formatCdf, formatIsoDay } from '@mungini/utils';
import { listCities, listTrips } from '@/services/trips/trips.api';
import type { City, Trip, TripPage } from '@/types/trips/trips';

const stateKeys: Record<string, string> = {
  REQUESTED: 'requested', PENDING_PAYMENT: 'pendingPayment', CONFIRMED: 'confirmed',
  CHECKED_IN: 'checkedIn', CHECKED_OUT: 'checkedOut', NO_SHOW: 'noShow',
  COMPLETED: 'completed', CANCELLED: 'cancelledState', EXPIRED: 'expired', DECLINED: 'declined', HELD: 'held',
};

export const useTrips = () => {
  const { client, status, city, setCity } = useSession();
  const { i18n } = useTranslation();
  const t = i18n.getFixedT(i18n.language, 'trips') as unknown as (key: string) => string;
  const [cities, setCities] = useState<City[]>([]);
  const [data, setData] = useState<TripPage | null>(null);
  const [page, setPage] = useState(1);
  const [tab, setTab] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(false);
  const [refresh, setRefresh] = useState(0);

  useEffect(() => {
    let active = true;
    void listCities(client).then((items) => { if (active) setCities(items); }).catch(() => { if (active) setError(true); });
    return () => { active = false; };
  }, [client]);

  useEffect(() => {
    if (status !== 'authenticated') return;
    let active = true;
    setLoading(true);
    setData(null);
    void listTrips(client, page, tab).then((result) => {
      if (active) { setData(result); setError(false); }
    }).catch(() => { if (active) setError(true); }).finally(() => { if (active) setLoading(false); });
    return () => { active = false; };
  }, [client, city, status, page, tab, refresh]);

  const name = (item: Trip) => i18n.language.startsWith('fr')
    ? item.propertyNameFr || item.propertyNameEn
    : item.propertyNameEn || item.propertyNameFr;
  const unit = (item: Trip) => i18n.language.startsWith('fr')
    ? item.unitNameFr || item.unitNameEn
    : item.unitNameEn || item.unitNameFr;
  const labels = {
    title: t('title'), signIn: t('signIn'), signInPrompt: t('signInPrompt'), city: t('city'), filter: t('filter'),
    loading: t('loading'), empty: t('empty'), error: t('error'), retry: t('retry'), previous: t('previous'),
    next: t('next'), page: t('page'), checkIn: t('checkIn'), checkOut: t('checkOut'), total: t('total'),
  };
  return {
    status, city, data, page, tab, loading, error, labels,
    cityOptions: cities.map((item) => ({ value: item.code, label: item.name })),
    tabOptions: ['', 'pending', 'upcoming', 'active', 'past', 'cancelled'].map((value) => ({ value: value || 'all', label: t(value || 'all') })),
    changeCity: (value: string) => { setPage(1); void setCity(value).catch(() => setError(true)); },
    changeTab: (value: string) => { setPage(1); setTab(value === 'all' ? '' : value); },
    previous: () => setPage((value) => Math.max(1, value - 1)),
    next: () => setPage((value) => value + 1),
    retry: () => setRefresh((value) => value + 1),
    name, unit, date: formatIsoDay,
    state: (item: Trip) => t(stateKeys[item.state] ?? 'unknown'),
    money: (item: Trip) => formatCdf(item.total, i18n.language),
  };
};
