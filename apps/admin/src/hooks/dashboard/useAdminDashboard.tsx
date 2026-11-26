import { useEffect, useMemo, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { addDays, formatCdf } from '@mungini/utils';
import type { TableColumn } from '@mungini/ui-web';
import { adminApi, type HostFigures } from '@/services/admin/admin.api';
import type { components } from '@mungini/api';

const PAGE_SIZE = 20;
const today = () => new Date().toISOString().slice(0, 10);

export const useAdminDashboard = () => {
  const { t, i18n } = useTranslation('admin');
  const translate = t as unknown as (key: string) => string;
  const { client, city } = useSession();
  const api = useMemo(() => adminApi(client), [client]);
  const [range, setRange] = useState<{ start: string | null; end: string | null }>({ start: addDays(today(), -29), end: today() });
  const [totals, setTotals] = useState<components['schemas']['DashboardSummary'] | null>(null);
  const [hosts, setHosts] = useState<HostFigures[]>([]);
  const [page, setPage] = useState(1);
  const [total, setTotal] = useState(0);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!range.start || !range.end) return;
    let active = true;
    setLoading(true);
    setError(null);
    void Promise.all([api.dashboard(range.start, range.end), api.hostFigures(range.start, range.end, page, PAGE_SIZE)]).then(([summary, hostPage]) => {
      if (!active) return;
      setLoading(false);
      if (summary.error || hostPage.error) {
        setError(t('error'));
        return;
      }
      setTotals(summary.data);
      setHosts(hostPage.data?.items ?? []);
      setTotal(hostPage.data?.total ?? 0);
    });
    return () => {
      active = false;
    };
  }, [api, range.start, range.end, page, city, t]);

  const columns: TableColumn<HostFigures>[] = [
    {
      key: 'host',
      header: t('hostName'),
      cell: (row) => (
        <div className="min-w-0">
          <p className="truncate font-medium text-foreground">{row.hostName}</p>
          <p className="truncate text-label text-muted-foreground">{row.city}</p>
        </div>
      ),
    },
    {
      key: 'revenue',
      header: t('revenue'),
      align: 'right',
      cell: (row) => <span className="tabular-nums">{formatCdf(row.revenue, i18n.language, 0)}</span>,
    },
    {
      key: 'bookings',
      header: t('bookings'),
      align: 'right',
      cell: (row) => <span className="tabular-nums">{row.bookings}</span>,
      hideBelow: 'sm',
    },
    {
      key: 'cancellations',
      header: t('cancellations'),
      align: 'right',
      cell: (row) => <span className="tabular-nums">{row.cancellations}</span>,
      hideBelow: 'md',
    },
  ];

  return {
    range,
    setRange,
    totals,
    hosts,
    page,
    setPage,
    total,
    pageSize: PAGE_SIZE,
    loading,
    error,
    columns,
    labels: {
      title: t('dashboardTitle'),
      subtitle: t('dashboardSubtitle'),
      revenue: t('revenue'),
      bookings: t('bookings'),
      cancellations: t('cancellations'),
      pending: t('pendingRequests'),
      openCases: t('openCases'),
      credit: t('creditBalance'),
      hosts: t('dashboardHosts'),
      error: t('error'),
      status: (status: string) => translate(`hostStatuses.${status}`),
    },
    values: {
      revenue: formatCdf(totals?.totals.revenue, i18n.language, 0),
      bookings: String(totals?.totals.bookings ?? 0),
      pending: String(totals?.totals.pendingRequests ?? 0),
      openCases: String(totals?.totals.openCases ?? 0),
      credit: formatCdf(totals?.totals.creditBalance, i18n.language, 0),
    },
  };
};
