import { useEffect, useMemo, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { useDebouncedValue, type TableColumn } from '@mungini/ui-web';
import { formatCdf, formatIsoDay } from '@mungini/utils';
import { hostApi } from '@/services/host/host.api';
import { listReservations } from '@/services/reservations/reservations.api';
import type { HostReservation, PropertyOption, ReservationState } from '@/types/reservations/reservations';

const PAGE_SIZE = 20;

const STATES: ReservationState[] = ['PENDING_PAYMENT', 'REQUESTED', 'CONFIRMED', 'DECLINED', 'CANCELLED', 'EXPIRED', 'CHECKED_IN', 'CHECKED_OUT', 'NO_SHOW'];

export const useHostReservations = () => {
  const { t, i18n } = useTranslation('hostReservations');
  const { t: th } = useTranslation('host');
  const translate = t as unknown as (key: string) => string;
  const translateHost = th as unknown as (key: string) => string;
  const { session, client } = useSession();
  const hosts = session?.hosts ?? [];
  const [hostId, setHostId] = useState(hosts[0]?.hostId ?? '');
  const [state, setState] = useState('all');
  const [propertyId, setPropertyId] = useState('all');
  const [search, setSearch] = useState('');
  const term = useDebouncedValue(search, 250);
  const [page, setPage] = useState(1);
  const [rows, setRows] = useState<HostReservation[]>([]);
  const [total, setTotal] = useState(0);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [properties, setProperties] = useState<PropertyOption[]>([]);
  const [selected, setSelected] = useState<string | null>(null);
  const [refreshToken, setRefreshToken] = useState(0);
  const locale = i18n.language.startsWith('fr') ? 'fr' : 'en';

  useEffect(() => {
    if (!hosts.some((host) => host.hostId === hostId)) setHostId(hosts[0]?.hostId ?? '');
  }, [hosts, hostId]);

  useEffect(() => {
    setPage(1);
  }, [hostId, state, propertyId, term]);

  useEffect(() => {
    setPropertyId('all');
    if (!hostId) {
      setProperties([]);
      return;
    }
    let active = true;
    void hostApi(client).properties(hostId, 1, 100, '').then((result) => {
      if (!active || result.error) return;
      const french = i18n.language.startsWith('fr');
      setProperties((result.data?.items ?? []).map((property) => ({
        value: property.id,
        label: (french ? property.nameFr : property.nameEn) || property.nameEn || property.nameFr,
      })));
    });
    return () => {
      active = false;
    };
  }, [client, hostId, i18n.language]);

  useEffect(() => {
    if (!hostId) return;
    let active = true;
    setLoading(true);
    setError(null);
    void listReservations(client, { hostId, propertyId: propertyId === 'all' ? '' : propertyId, state: state === 'all' ? '' : state, search: term, page, pageSize: PAGE_SIZE, locale }).then((result) => {
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
  }, [client, hostId, propertyId, state, term, page, locale, refreshToken, translate]);

  const stateOptions = useMemo(
    () => [{ value: 'all', label: translate('allStates') }, ...STATES.map((value) => ({ value, label: translate(`states.${value}`) }))],
    [translate],
  );
  const propertyOptions = useMemo(
    () => [{ value: 'all', label: translate('allProperties') }, ...properties],
    [properties, translate],
  );

  const stateLabel = (value: string) => translate(`states.${value}`);
  const columns: TableColumn<HostReservation>[] = [
    {
      key: 'reservation',
      header: translate('colReservation'),
      cell: (row) => (
        <div className="min-w-0">
          <p className="truncate font-medium text-foreground">{row.reference}</p>
          <p className="truncate text-label text-muted-foreground">{row.propertyName}</p>
          <p className="text-label text-muted-foreground sm:hidden">{stateLabel(row.state)}</p>
        </div>
      ),
    },
    {
      key: 'stay',
      header: translate('colStay'),
      hideBelow: 'sm',
      cell: (row) => (
        <div className="min-w-0">
          <p className="whitespace-nowrap tabular-nums text-foreground">{formatIsoDay(row.checkIn)} – {formatIsoDay(row.checkOut)}</p>
          <p className="text-label text-muted-foreground">{row.nights.length} {translate('nights').toLowerCase()}</p>
        </div>
      ),
    },
    {
      key: 'guest',
      header: translate('colGuest'),
      hideBelow: 'sm',
      cell: (row) => row.guest?.fullName ?? '',
    },
    {
      key: 'status',
      header: translate('colStatus'),
      hideBelow: 'sm',
      cell: (row) => stateLabel(row.state),
    },
    {
      key: 'total',
      header: translate('colTotal'),
      align: 'right',
      cell: (row) => <span className="tabular-nums">{formatCdf(row.total, i18n.language)}</span>,
    },
  ];

  return {
    hosts,
    hasHost: hosts.length > 0,
    hostId,
    setHostId,
    state,
    setState,
    propertyId,
    setPropertyId,
    search,
    setSearch,
    page,
    setPage,
    pageSize: PAGE_SIZE,
    rows,
    total,
    loading,
    error,
    columns,
    stateOptions,
    propertyOptions,
    selected,
    setSelected,
    refresh: () => setRefreshToken((value) => value + 1),
    labels: {
      title: translate('title'),
      subtitle: translate('subtitle'),
      search: translate('search'),
      statusFilter: translate('statusFilter'),
      propertyFilter: translate('propertyFilter'),
      empty: translate('empty'),
      noHost: translateHost('noHost'),
      noHostAction: translateHost('noHostAction'),
      loading: translate('loading'),
    },
  };
};
