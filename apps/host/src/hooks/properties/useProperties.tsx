import { useEffect, useMemo, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { useDebouncedValue, type TableColumn } from '@mungini/ui-web';
import { hostApi, type PropertyView } from '@/services/host/host.api';
import { createProperty, listCities, propertyLifecycle } from '@/services/properties/properties.api';
import { propertyStatusView } from '@/utils/status/propertyStatus';

const PAGE_SIZE = 20;

export const useProperties = () => {
  const { t, i18n } = useTranslation('host');
  const translate = t as unknown as (key: string) => string;
  const { session, client, city } = useSession();
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
  const [notice, setNotice] = useState<string | null>(null);
  const [selected, setSelected] = useState<PropertyView | null>(null);
  const [createOpen, setCreateOpen] = useState(false);
  const [nameEn, setNameEn] = useState('');
  const [nameFr, setNameFr] = useState('');
  const [addressLine, setAddressLine] = useState('');
  const [descriptionEn, setDescriptionEn] = useState('');
  const [descriptionFr, setDescriptionFr] = useState('');
  const [timezone, setTimezone] = useState('');
  const [busy, setBusy] = useState(false);
  const [formError, setFormError] = useState<string | null>(null);
  const [action, setAction] = useState<'PUBLISH' | 'PAUSE' | 'ARCHIVE' | null>(null);
  const [lifecycleBusy, setLifecycleBusy] = useState(false);
  const [lifecycleError, setLifecycleError] = useState<string | null>(null);
  const [blockers, setBlockers] = useState<string[]>([]);

  useEffect(() => {
    if (!hosts.some((host) => host.hostId === hostId)) setHostId(hosts[0]?.hostId ?? '');
  }, [hosts, hostId]);

  useEffect(() => {
    setPage(1);
  }, [hostId, term]);

  useEffect(() => {
    let active = true;
    void listCities(client).then(({ data }) => {
      const match = data?.data.find((item) => item.code === city);
      if (active) setTimezone(match?.timezone ?? '');
    }).catch(() => undefined);
    return () => {
      active = false;
    };
  }, [client, city]);

  useEffect(() => {
    if (!hostId) return;
    let active = true;
    setLoading(true);
    setError(null);
    void api.properties(hostId, page, PAGE_SIZE, term).then((result) => {
      if (!active) return;
      setLoading(false);
      if (result.error) {
        setError(translate('error'));
        return;
      }
      setItems(result.data?.items ?? []);
      setTotal(result.data?.total ?? 0);
    });
    return () => {
      active = false;
    };
  }, [api, hostId, page, term, translate]);

  const refresh = async () => {
    const result = await api.properties(hostId, page, PAGE_SIZE, term);
    setItems(result.data?.items ?? []);
    setTotal(result.data?.total ?? 0);
  };

  const openCreate = () => {
    setNameEn('');
    setNameFr('');
    setAddressLine('');
    setDescriptionEn('');
    setDescriptionFr('');
    setFormError(null);
    setCreateOpen(true);
  };

  const submitCreate = async () => {
    if (!hostId || !timezone) return;
    setBusy(true);
    setFormError(null);
    const result = await createProperty(client, { hostId, nameEn, nameFr, timezone, addressLine, descriptionEn, descriptionFr });
    setBusy(false);
    if (result.error || !result.data) {
      setFormError(translate('error'));
      return;
    }
    setCreateOpen(false);
    setNotice(translate('savedProperty'));
    await refresh();
    setSelected(result.data);
  };

  const openLifecycle = (next: 'PUBLISH' | 'PAUSE' | 'ARCHIVE') => {
    setAction(next);
    setLifecycleError(null);
    setBlockers([]);
  };

  const submitLifecycle = async () => {
    if (!selected || !action) return;
    setLifecycleBusy(true);
    setLifecycleError(null);
    setBlockers([]);
    const result = await propertyLifecycle(client, selected.id, action, selected.version);
    setLifecycleBusy(false);
    if (result.error || !result.data) {
      setAction(null);
      setLifecycleError(translate('error'));
      setBlockers(result.blockers);
      return;
    }
    setAction(null);
    setNotice(translate('lifecycleSaved'));
    setSelected(result.data);
    await refresh();
  };

  const statusLabel = (status: string) => propertyStatusView(status, translate);
  const columns: TableColumn<PropertyView>[] = [
    {
      key: 'name',
      header: translate('propertyName'),
      cell: (row) => {
        const name = i18n.language === 'fr' ? row.nameFr : row.nameEn;
        const secondary = i18n.language === 'fr' ? row.nameEn : row.nameFr;
        return (
          <div className="min-w-0">
            <p className="truncate font-medium text-foreground">{name || secondary}</p>
            <p className="truncate text-label text-muted-foreground">{row.addressLine}</p>
            <p className="text-label text-muted-foreground sm:hidden">{statusLabel(row.status)}</p>
          </div>
        );
      },
    },
    { key: 'status', header: translate('propertyStatus'), cell: (row) => statusLabel(row.status), align: 'right', hideBelow: 'sm' },
    { key: 'city', header: translate('propertyCity'), cell: (row) => row.city, hideBelow: 'sm', align: 'right' },
  ];

  return {
    hosts, hostId, setHostId, search, setSearch, page, setPage, pageSize: PAGE_SIZE, items, total, loading, error, notice, columns,
    selected, setSelected, createOpen, setCreateOpen, openCreate, nameEn, setNameEn, nameFr, setNameFr, addressLine, setAddressLine,
    descriptionEn, setDescriptionEn, descriptionFr, setDescriptionFr, timezone, busy, formError, submitCreate,
    action, setAction, openLifecycle, submitLifecycle, lifecycleBusy, lifecycleError, blockers,
    blockerLabels: blockers.map((code) => {
      const key = `blockerLabels.${code}`;
      const label = translate(key);
      return label === key ? code : label;
    }),
    statusText: (status: string) => translate(`propertyStatuses.${status}`),
    nameFor: (property: PropertyView) => {
      const primary = i18n.language === 'fr' ? property.nameFr : property.nameEn;
      return primary || property.nameEn || property.nameFr;
    },
    createReady: Boolean(nameEn.trim() && nameFr.trim() && addressLine.trim() && timezone),
    labels: {
      title: translate('propertiesTitle'), subtitle: translate('propertiesSubtitle'), empty: translate('propertiesEmpty'), search: translate('propertiesTitle'),
      newProperty: translate('newProperty'), createProperty: translate('createProperty'), propertyDetails: translate('propertyDetails'),
      nameEn: translate('propertyName'), nameFr: translate('propertyName'), address: translate('propertyAddress'),
      descriptionEn: translate('descriptionEn'), descriptionFr: translate('descriptionFr'), timezone: translate('timezone'), create: translate('create'),
      units: translate('units'), unitTypes: translate('unitTypes'), photos: translate('photos'), openReservations: translate('openReservations'),
      pendingRequests: translate('pendingRequestCount'), publishedRevision: translate('publishedRevision'), guestContact: translate('guestContact'),
      status: translate('propertyStatus'), city: translate('propertyCity'), publish: translate('publish'), pause: translate('pause'), archive: translate('archive'),
      close: translate('close'), confirmPublishTitle: translate('confirmPublishTitle'), confirmPublishBody: translate('confirmPublishBody'),
      confirmPauseTitle: translate('confirmPauseTitle'), confirmPauseBody: translate('confirmPauseBody'), confirmArchiveTitle: translate('confirmArchiveTitle'),
      confirmArchiveBody: translate('confirmArchiveBody'), blockers: translate('blockers'), saved: translate('savedProperty'), error: translate('error'),
    },
  };
};

export type PropertiesView = ReturnType<typeof useProperties>;
