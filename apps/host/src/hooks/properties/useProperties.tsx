import { useEffect, useMemo, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { useDebouncedValue, type TableColumn } from '@mungini/ui-web';
import { hostApi, type PropertyView } from '@/services/host/host.api';
import { propertyStatusView } from '@/utils/status/propertyStatus';

const PAGE_SIZE = 20;

export const useProperties = () => {
  const { t, i18n } = useTranslation('host');
  const translate = t as unknown as (key: string) => string;
  const { session, client } = useSession();
  const api = useMemo(() => hostApi(client), [client]);
  const hosts = session?.hosts ?? [];
  const [hostId, setHostId] = useState(hosts[0]?.hostId ?? '');
  const [search, setSearch] = useState('');
  const term = useDebouncedValue(search, 250);
  const [page, setPage] = useState(1);
  const [items, setItems] = useState<PropertyView[]>([]);
  const [total, setTotal] = useState(0);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!hostId && hosts[0]) setHostId(hosts[0].hostId);
  }, [hosts, hostId]);

  useEffect(() => {
    setPage(1);
  }, [hostId, term]);

  useEffect(() => {
    if (!hostId) return;
    let active = true;
    setLoading(true);
    setError(null);
    void api.properties(hostId, page, PAGE_SIZE, term).then((result) => {
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
  }, [api, hostId, page, term, t]);

  const columns: TableColumn<PropertyView>[] = [
    {
      key: 'name',
      header: t('propertyName'),
      cell: (row) => {
        const name = i18n.language === 'fr' ? row.nameFr : row.nameEn;
        const secondary = i18n.language === 'fr' ? row.nameEn : row.nameFr;
        return (
          <div className="min-w-0">
            <p className="truncate font-medium text-foreground">{name || secondary}</p>
            <p className="truncate text-label text-muted-foreground">{row.addressLine}</p>
          </div>
        );
      },
    },
    {
      key: 'status',
      header: t('propertyStatus'),
      cell: (row) => propertyStatusView(row.status, translate),
      align: 'right',
    },
    {
      key: 'city',
      header: t('propertyCity'),
      cell: (row) => row.city,
      hideBelow: 'sm',
      align: 'right',
    },
  ];

  return {
    hosts,
    hostId,
    setHostId,
    search,
    setSearch,
    page,
    setPage,
    pageSize: PAGE_SIZE,
    items,
    total,
    loading,
    error,
    columns,
    labels: {
      title: t('propertiesTitle'),
      subtitle: t('propertiesSubtitle'),
      empty: t('propertiesEmpty'),
      search: t('propertiesTitle'),
    },
  };
};
