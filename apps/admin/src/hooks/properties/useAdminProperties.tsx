import { useEffect, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { useDebouncedValue, type TableColumn } from '@mungini/ui-web';
import { listProperties } from '@/services/properties/properties.api';
import type { AdminProperty } from '@/types/properties/properties';

const PAGE_SIZE = 20;

export const useAdminProperties = () => {
  const { t, i18n } = useTranslation('adminProperties');
  const translate = t as unknown as (key: string) => string;
  const { client, city } = useSession();
  const [search, setSearch] = useState('');
  const term = useDebouncedValue(search, 250);
  const [status, setStatus] = useState('all');
  const [page, setPage] = useState(1);
  const [rows, setRows] = useState<AdminProperty[]>([]);
  const [total, setTotal] = useState(0);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [notice, setNotice] = useState<string | null>(null);
  const [selected, setSelected] = useState<string | null>(null);
  const [refreshToken, setRefreshToken] = useState(0);
  const french = i18n.language.startsWith('fr');

  useEffect(() => {
    setPage(1);
  }, [term, status, city]);

  useEffect(() => {
    let active = true;
    setLoading(true);
    setError(null);
    void listProperties(client, status === 'all' ? '' : status, term, page, PAGE_SIZE).then((result) => {
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
  }, [client, city, status, term, page, refreshToken, translate]);

  const name = (row: AdminProperty) => (french ? row.nameFr : row.nameEn) || row.nameEn || row.nameFr;
  const statusLabel = (value: string) => {
    const key = `statuses.${value}`;
    const label = translate(key);
    return label === key ? value : label;
  };
  const columns: TableColumn<AdminProperty>[] = [
    {
      key: 'property',
      header: translate('property'),
      cell: (row) => (
        <div className="min-w-0">
          <p className="truncate font-medium text-foreground">{name(row)}</p>
          <p className="truncate text-label text-muted-foreground">{row.hostName} · {row.neighborhood}</p>
          <p className="text-label text-muted-foreground sm:hidden">{statusLabel(row.status)}</p>
        </div>
      ),
    },
    { key: 'status', header: translate('status'), hideBelow: 'sm', cell: (row) => statusLabel(row.status) },
    { key: 'revisions', header: translate('openRevisions'), align: 'right', hideBelow: 'sm', cell: (row) => <span className="tabular-nums">{row.openRevisions}</span> },
    { key: 'units', header: translate('unitTypes'), align: 'right', hideBelow: 'sm', cell: (row) => <span className="tabular-nums">{row.unitTypeCount} / {row.unitCount}</span> },
    { key: 'city', header: translate('city'), hideBelow: 'sm', cell: (row) => row.city },
  ];

  return {
    search, setSearch, status, setStatus, page, setPage, pageSize: PAGE_SIZE, rows, total, loading, error, notice, columns,
    selected, setSelected,
    onChanged: () => setRefreshToken((value) => value + 1),
    statusLabel,
    name,
    statusOptions: [
      { value: 'all', label: translate('allStatuses') },
      ...['DRAFT', 'APPROVED', 'PUBLISHED', 'PAUSED', 'SUSPENDED', 'ARCHIVED'].map((value) => ({ value, label: translate(`statuses.${value}`) })),
    ],
    labels: {
      title: translate('title'), subtitle: translate('subtitle'), search: translate('search'), status: translate('status'),
      empty: translate('empty'), error: translate('error'),
    },
  };
};

export type AdminPropertiesView = ReturnType<typeof useAdminProperties>;
