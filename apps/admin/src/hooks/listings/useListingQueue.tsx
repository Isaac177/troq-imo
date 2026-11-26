import { useEffect, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import type { TableColumn } from '@mungini/ui-web';
import { formatDateTime } from '@mungini/utils';
import { listCities, listQueue } from '@/services/listings/listings.api';
import type { QueueItem } from '@/types/listings/listings';

const PAGE_SIZE = 20;

export const useListingQueue = () => {
  const { t } = useTranslation('adminListings');
  const translate = t as unknown as (key: string) => string;
  const { client, city } = useSession();
  const [status, setStatus] = useState('SUBMITTED');
  const [page, setPage] = useState(1);
  const [rows, setRows] = useState<QueueItem[]>([]);
  const [total, setTotal] = useState(0);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [notice, setNotice] = useState<string | null>(null);
  const [selected, setSelected] = useState<string | null>(null);
  const [timezone, setTimezone] = useState('UTC');
  const [refreshToken, setRefreshToken] = useState(0);

  useEffect(() => {
    setPage(1);
  }, [status, city]);

  useEffect(() => {
    let active = true;
    void listCities(client).then(({ data }) => {
      const match = data?.data.find((item) => item.code === city);
      if (active && match) setTimezone(match.timezone);
    }).catch(() => undefined);
    return () => {
      active = false;
    };
  }, [client, city]);

  useEffect(() => {
    let active = true;
    setLoading(true);
    setError(null);
    void listQueue(client, status === 'all' ? '' : status, page, PAGE_SIZE).then((result) => {
      if (!active) return;
      setLoading(false);
      if (result.error || !result.data) {
        setError(translate('error'));
        setRows([]);
        setTotal(0);
        return;
      }
      setRows(result.data.items);
      setTotal(result.data.total);
    });
    return () => {
      active = false;
    };
  }, [client, city, status, page, refreshToken, translate]);

  const columns: TableColumn<QueueItem>[] = [
    {
      key: 'property',
      header: translate('property'),
      cell: (row) => (
        <div className="min-w-0">
          <p className="truncate font-medium text-foreground">{row.propertyNameEn}</p>
          <p className="truncate text-label text-muted-foreground">{row.hostName} · {translate('revision')} {row.revisionNumber}</p>
          <p className="text-label text-muted-foreground sm:hidden">{translate(`statuses.${row.reviewStatus}`)}</p>
        </div>
      ),
    },
    { key: 'status', header: translate('status'), hideBelow: 'sm', cell: (row) => translate(`statuses.${row.reviewStatus}`) },
    { key: 'changed', header: translate('changedFields'), align: 'right', hideBelow: 'sm', cell: (row) => <span className="tabular-nums">{row.changedFields}</span> },
    { key: 'submitted', header: translate('submittedAt'), hideBelow: 'sm', cell: (row) => <span className="whitespace-nowrap tabular-nums">{row.submittedAt ? formatDateTime(row.submittedAt, timezone) : '—'}</span> },
  ];

  return {
    status, setStatus,
    page, setPage, pageSize: PAGE_SIZE, rows, total, loading, error, notice, columns,
    selected, setSelected,
    onChanged: () => setRefreshToken((value) => value + 1),
    statusOptions: [
      { value: 'all', label: translate('allStatuses') },
      ...['SUBMITTED', 'CHANGES_REQUESTED', 'APPROVED', 'REJECTED'].map((value) => ({ value, label: translate(`statuses.${value}`) })),
    ],
    labels: {
      title: translate('title'), subtitle: translate('subtitle'), status: translate('status'), property: translate('property'),
      empty: translate('empty'), error: translate('error'),
    },
  };
};
