import { useEffect, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { formatCdf, formatDateTime, formatIsoDay } from '@mungini/utils';
import { getReservation, listCities } from '@/services/reservations/reservations.api';
import type { ReservationDetail } from '@/types/reservations/reservations';

export const useReservationDetails = (id: string | null) => {
  const { client, city } = useSession();
  const { t, i18n } = useTranslation('adminReservations');
  const tr = t as unknown as (key: string) => string;
  const [page, setPage] = useState(1);
  const [detail, setDetail] = useState<ReservationDetail | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(false);
  const [timezone, setTimezone] = useState('UTC');
  useEffect(() => { setPage(1); setDetail(null); }, [id, city]);
  useEffect(() => {
    if (!id) return;
    let active = true;
    setLoading(true);
    setError(false);
    void getReservation(client, id, page).then(({ data, error: failure }) => {
      if (!active) return;
      setDetail(data?.data ?? null);
      setError(Boolean(failure || !data));
    }).catch(() => { if (active) { setDetail(null); setError(true); } }).finally(() => { if (active) setLoading(false); });
    return () => { active = false; };
  }, [client, id, city, page]);
  useEffect(() => {
    if (!detail) return;
    let active = true;
    void listCities(client).then(({ data }) => {
      const match = data?.data.find((item) => item.code === detail.city);
      if (active && match) setTimezone(match.timezone);
    }).catch(() => undefined);
    return () => { active = false; };
  }, [client, detail]);
  const french = i18n.language.startsWith('fr');
  const maxTotal = detail ? Math.max(detail.items.total, detail.guests.total, detail.cases.total, detail.amendments.total, detail.timeline.total) : 0;
  const pageCount = Math.max(1, Math.ceil(maxTotal / 10));
  const stateLabels: Record<string, string> = { PENDING_PAYMENT: tr('pendingPayment'), REQUESTED: tr('requested'), CONFIRMED: tr('confirmed'), DECLINED: tr('declined'), CANCELLED: tr('cancelled'), EXPIRED: tr('expired'), CHECKED_IN: tr('checkedIn'), CHECKED_OUT: tr('checkedOut'), NO_SHOW: tr('noShow') };
  const actionLabel = (action: string) => {
    const key = `timelineActions.${action}`;
    const value = tr(key);
    return value === key ? tr('timelineEvent') : value;
  };
  return {
    detail, page, setPage, pageCount, loading, error, french,
    state: (value: string) => stateLabels[value] ?? tr('other'),
    caseType: (value: string) => {
      const key = `caseTypes.${value}`;
      const label = tr(key);
      return label === key ? value : label;
    },
    caseState: (value: string) => {
      const key = `caseStates.${value}`;
      const label = tr(key);
      return label === key ? value : label;
    },
    amendmentState: (value: string) => {
      const key = `amendmentStates.${value}`;
      const label = tr(key);
      return label === key ? value : label;
    },
    actionLabel,
    money: (value: string) => formatCdf(value, i18n.language), day: formatIsoDay, datetime: (value: string | null | undefined) => (value ? formatDateTime(value, timezone) : ''),
    labels: { title: tr('details'), loading: tr('loading'), error: tr('error'), host: tr('host'), property: tr('property'), status: tr('status'), amount: tr('amount'), commission: tr('commission'), net: tr('net'), stays: tr('stays'), guests: tr('guests'), cases: tr('cases'), amendments: tr('amendments'), timeline: tr('timeline'), empty: tr('empty'), previous: tr('previous'), next: tr('next'), page: tr('page'), adults: tr('adults'), children: tr('children'), quantity: tr('quantity'), noCases: tr('noCases'), noAmendments: tr('noAmendments'), noTimeline: tr('noTimeline'), caseDeadline: tr('caseDeadline'), amendmentExpires: tr('amendmentExpires'), amendmentDelta: tr('amendmentDelta'), amendmentResponded: tr('amendmentResponded') },
  };
};
