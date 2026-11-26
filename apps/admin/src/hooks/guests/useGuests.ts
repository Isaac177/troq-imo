import { useEffect, useMemo, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { useDebouncedValue } from '@mungini/ui-web';
import { adminApi, type GuestItem } from '@/services/admin/admin.api';

const PAGE_SIZE = 20;

export const useGuests = () => {
  const { t } = useTranslation('admin');
  const { client } = useSession();
  const api = useMemo(() => adminApi(client), [client]);
  const [search, setSearch] = useState('');
  const term = useDebouncedValue(search, 250);
  const [flag, setFlag] = useState('');
  const [page, setPage] = useState(1);
  const [items, setItems] = useState<GuestItem[]>([]);
  const [total, setTotal] = useState(0);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [selected, setSelected] = useState<GuestItem | null>(null);

  useEffect(() => {
    setPage(1);
  }, [term, flag]);

  useEffect(() => {
    let active = true;
    setLoading(true);
    setError(null);
    void api.guests(term, flag, page, PAGE_SIZE).then((result) => {
      if (!active) return;
      setLoading(false);
      if (result.error) {
        setError(t('error'));
        return;
      }
      setItems(result.data?.items ?? []);
      setTotal(result.data?.total ?? 0);
    });
    return () => {
      active = false;
    };
  }, [api, term, flag, page, t]);

  const open = async (row: GuestItem) => {
    const result = await api.guest(row.userId);
    if (result.error) {
      setError(t('error'));
      return;
    }
    setSelected(result.data);
  };

  return {
    search,
    setSearch,
    flag,
    setFlag,
    page,
    setPage,
    pageSize: PAGE_SIZE,
    items,
    total,
    loading,
    error,
    selected,
    open,
    close: () => setSelected(null),
    labels: {
      title: t('guestsTitle'),
      subtitle: t('guestsSubtitle'),
      guest: t('guest'),
      contact: t('contact'),
      bookings: t('totalBookings'),
      active: t('activeBookings'),
      cancelled: t('cancelledBookings'),
      spent: t('totalSpent'),
      lastBooking: t('lastBooking'),
      search: t('search'),
      all: t('allStatuses'),
      cases: t('openCases'),
      cancellations: t('cancellations'),
      empty: t('guestsEmpty'),
      error: t('error'),
    },
  };
};
