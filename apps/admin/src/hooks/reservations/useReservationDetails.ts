import { useEffect, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { formatCdf, formatIsoDay } from '@mungini/utils';
import { getReservation } from '@/services/reservations/reservations.api';
import type { ReservationDetail } from '@/types/reservations/reservations';

export const useReservationDetails = (id: string | null) => {
  const { client, city } = useSession();
  const { t, i18n } = useTranslation('adminReservations');
  const [page, setPage] = useState(1);
  const [detail, setDetail] = useState<ReservationDetail | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(false);
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
  const french = i18n.language.startsWith('fr');
  const maxTotal = detail ? Math.max(detail.items.total, detail.guests.total) : 0;
  const pageCount = Math.max(1, Math.ceil(maxTotal / 10));
  const stateLabels: Record<string, string> = { PENDING_PAYMENT: t('pendingPayment'), REQUESTED: t('requested'), CONFIRMED: t('confirmed'), DECLINED: t('declined'), CANCELLED: t('cancelled'), EXPIRED: t('expired'), CHECKED_IN: t('checkedIn'), CHECKED_OUT: t('checkedOut'), NO_SHOW: t('noShow') };
  return {
    detail, page, setPage, pageCount, loading, error, french,
    state: (value: string) => stateLabels[value] ?? t('other'),
    money: (value: string) => formatCdf(value, i18n.language), day: formatIsoDay,
    labels: { title: t('details'), loading: t('loading'), error: t('error'), host: t('host'), property: t('property'), status: t('status'), amount: t('amount'), commission: t('commission'), net: t('net'), stays: t('stays'), guests: t('guests'), cases: t('cases'), amendments: t('amendments'), timeline: t('timeline'), empty: t('empty'), previous: t('previous'), next: t('next'), page: t('page'), adults: t('adults'), children: t('children'), quantity: t('quantity') },
  };
};
