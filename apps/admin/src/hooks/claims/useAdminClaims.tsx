import { useEffect, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { useDebouncedValue, type TableColumn } from '@mungini/ui-web';
import { formatCdf, formatDateTime } from '@mungini/utils';
import { getClaimSummary, listCities, listClaims } from '@/services/claims/claims.api';
import type { ClaimRow, ClaimSummary } from '@/types/claims/claims';

const PAGE_SIZE = 20;

export const useAdminClaims = () => {
  const { t, i18n } = useTranslation('adminClaims');
  const translate = t as unknown as (key: string) => string;
  const { client, city } = useSession();
  const [search, setSearch] = useState('');
  const term = useDebouncedValue(search, 250);
  const [status, setStatus] = useState('all');
  const [page, setPage] = useState(1);
  const [rows, setRows] = useState<ClaimRow[]>([]);
  const [summary, setSummary] = useState<ClaimSummary | null>(null);
  const [total, setTotal] = useState(0);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [notice, setNotice] = useState<string | null>(null);
  const [selected, setSelected] = useState<string | null>(null);
  const [timezone, setTimezone] = useState('UTC');
  const [refreshToken, setRefreshToken] = useState(0);
  const french = i18n.language.startsWith('fr');

  useEffect(() => {
    setPage(1);
  }, [term, status, city]);

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
    const query = { status: status === 'all' ? '' : status, search: term, page, pageSize: PAGE_SIZE };
    void Promise.all([listClaims(client, query), getClaimSummary(client, query)]).then(([list, summaryResult]) => {
      if (!active) return;
      setLoading(false);
      if (list.error || !list.data || summaryResult.error || !summaryResult.data) {
        setError(translate('error'));
        setRows([]);
        setTotal(0);
        return;
      }
      setRows(list.data.items);
      setTotal(list.data.total);
      setSummary(summaryResult.data);
    });
    return () => {
      active = false;
    };
  }, [client, city, status, term, page, refreshToken, translate]);

  const money = (value: string | null | undefined) => (value ? formatCdf(value, i18n.language) : '—');
  const propertyName = (row: ClaimRow) => (french ? row.propertyNameFr : row.propertyNameEn) || row.propertyNameEn || row.propertyNameFr || '';
  const columns: TableColumn<ClaimRow>[] = [
    {
      key: 'claim',
      header: translate('property'),
      cell: (row) => (
        <div className="min-w-0">
          <p className="truncate font-medium text-foreground">{propertyName(row)}</p>
          <p className="truncate text-label text-muted-foreground">{row.operatorName}{row.reservationReference ? ` · ${row.reservationReference}` : ''}</p>
          <p className="truncate text-label text-muted-foreground sm:hidden">{row.guestName || ''}</p>
        </div>
      ),
    },
    { key: 'amount', header: translate('claimAmount'), align: 'right', cell: (row) => <span className="tabular-nums">{money(row.claimAmount)}</span> },
    { key: 'decision', header: translate('decision'), hideBelow: 'sm', cell: (row) => (row.decision ? translate(`decisions.${row.decision}`) : translate('decisions.UNDECIDED')) },
    { key: 'payout', header: translate('payout'), hideBelow: 'sm', cell: (row) => (row.payoutState ? translate(`payoutStates.${row.payoutState}`) : '—') },
    { key: 'deadline', header: translate('deadline'), hideBelow: 'sm', cell: (row) => <span className="whitespace-nowrap tabular-nums">{formatDateTime(row.deadlineAt, timezone)}</span> },
  ];

  return {
    search, setSearch, status, setStatus, page, setPage, pageSize: PAGE_SIZE, rows, summary, total, loading, error, notice, columns,
    selected, setSelected,
    onChanged: () => setRefreshToken((value) => value + 1),
    money,
    statusOptions: [
      { value: 'all', label: translate('allStatuses') },
      ...['UNDECIDED', 'APPROVED', 'PARTIAL', 'REJECTED'].map((value) => ({ value, label: translate(`decisions.${value}`) })),
    ],
    labels: {
      title: translate('title'), subtitle: translate('subtitle'), search: translate('search'), status: translate('status'),
      total: translate('total'), undecided: translate('undecided'), claimed: translate('claimed'), approved: translate('approved'),
      paid: translate('paid'), payoutPending: translate('payoutPending'), empty: translate('empty'), error: translate('error'),
    },
  };
};

export type AdminClaimsView = ReturnType<typeof useAdminClaims>;
