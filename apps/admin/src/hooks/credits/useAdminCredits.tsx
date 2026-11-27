import { useEffect, useState, type ChangeEvent } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { StatusDot, useDebouncedValue, type TableColumn } from '@mungini/ui-web';
import { formatCdf, formatDateTime } from '@mungini/utils';
import { getCreditSummary, listCreditAccounts } from '@/services/credits/credits.api';
import type { CreditAccount, CreditStatus, CreditSummary } from '@/types/credits/credits';
import { useCityTimezone } from '@/hooks/credits/useCityTimezone';

const PAGE_SIZE = 20;

const stateOf = (row: CreditAccount): CreditStatus => (!row.enabled ? 'DISABLED' : row.low ? 'LOW' : 'ENABLED');

const toneOf = (state: CreditStatus): 'success' | 'warning' | 'danger' => (state === 'DISABLED' ? 'danger' : state === 'LOW' ? 'warning' : 'success');

export const useAdminCredits = () => {
  const { t, i18n } = useTranslation('adminCredits');
  const translate = t as unknown as (key: string) => string;
  const { client, city } = useSession();
  const { timezone } = useCityTimezone();
  const [search, setSearch] = useState('');
  const term = useDebouncedValue(search, 250);
  const [status, setStatus] = useState('all');
  const [page, setPage] = useState(1);
  const [rows, setRows] = useState<CreditAccount[]>([]);
  const [summary, setSummary] = useState<CreditSummary | null>(null);
  const [total, setTotal] = useState(0);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [forbidden, setForbidden] = useState(false);
  const [selected, setSelected] = useState<string | null>(null);
  const [refreshToken, setRefreshToken] = useState(0);

  useEffect(() => {
    setPage(1);
    setSelected(null);
  }, [term, status, city]);

  useEffect(() => {
    let active = true;
    setLoading(true);
    setError(null);
    setForbidden(false);
    const query = { status: status === 'all' ? '' : status, search: term, page, pageSize: PAGE_SIZE };
    void Promise.all([listCreditAccounts(client, query), getCreditSummary(client)]).then(([list, summaryResult]) => {
      if (!active) return;
      setLoading(false);
      if (list.error || !list.data || summaryResult.error || !summaryResult.data) {
        const code = list.error ?? summaryResult.error;
        setForbidden(code === 'PERMISSION_MISSING');
        setError(code === 'PERMISSION_MISSING' ? translate('forbidden') : translate('error'));
        setRows([]);
        setTotal(0);
        setSummary(null);
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

  const money = (value: string | number | null | undefined) => (value === null || value === undefined ? '—' : formatCdf(value, i18n.language));
  const percent = (bps: number) => new Intl.NumberFormat(i18n.language, { style: 'percent', minimumFractionDigits: 0, maximumFractionDigits: 2 }).format(bps / 10000);
  const stateLabel = (value: CreditStatus) => translate(`states.${value}`);

  const columns: TableColumn<CreditAccount>[] = [
    {
      key: 'host',
      header: translate('host'),
      cell: (row) => (
        <div className="min-w-0">
          <p className="truncate font-medium text-foreground">{row.hostName}</p>
          <div className="mt-0.5 sm:hidden">
            <StatusDot tone={toneOf(stateOf(row))} label={stateLabel(stateOf(row))} />
          </div>
        </div>
      ),
    },
    {
      key: 'balance',
      header: translate('balance'),
      align: 'right',
      cell: (row) => <span className={stateOf(row) === 'ENABLED' ? 'tabular-nums' : stateOf(row) === 'LOW' ? 'tabular-nums text-tone-attention' : 'tabular-nums text-tone-problem'}>{money(row.balance)}</span>,
    },
    {
      key: 'state',
      header: translate('state'),
      hideBelow: 'sm',
      cell: (row) => <StatusDot tone={toneOf(stateOf(row))} label={stateLabel(stateOf(row))} />,
    },
    {
      key: 'rate',
      header: translate('commissionRate'),
      align: 'right',
      hideBelow: 'sm',
      cell: (row) => <span className="tabular-nums">{percent(row.rateBps)}</span>,
    },
    {
      key: 'lastTopUp',
      header: translate('lastTopUp'),
      hideBelow: 'sm',
      cell: (row) => <span className="whitespace-nowrap tabular-nums">{row.lastTopUpAt ? formatDateTime(row.lastTopUpAt, timezone) : '—'}</span>,
    },
  ];

  const selectedName = rows.find((row) => row.hostId === selected)?.hostName ?? '';

  return {
    search, status, setStatus, page, setPage, pageSize: PAGE_SIZE, rows, total, loading, error, forbidden,
    changeSearch: (event: ChangeEvent<HTMLInputElement>) => setSearch(event.target.value),
    columns, selected, selectedName, rowId: (row: CreditAccount) => row.hostId,
    tableKey: `${status}:${term}:${page}`,
    summaryLoading: loading && !summary,
    openRow: (row: CreditAccount) => setSelected(row.hostId),
    closeDrawer: () => setSelected(null),
    onChanged: () => setRefreshToken((value) => value + 1),
    statusOptions: [
      { value: 'all', label: translate('allStatuses') },
      ...(['ENABLED', 'LOW', 'DISABLED'] as CreditStatus[]).map((value) => ({ value, label: translate(`states.${value}`) })),
    ],
    summaryCards: summary
      ? [
          { key: 'accounts', label: translate('summary.accounts'), value: String(summary.accounts) },
          { key: 'totalBalance', label: translate('summary.totalBalance'), value: money(summary.totalBalance) },
          { key: 'low', label: translate('summary.low'), value: String(summary.low) },
          { key: 'disabled', label: translate('summary.disabled'), value: String(summary.disabled) },
          { key: 'commissionTaken30d', label: translate('summary.commissionTaken30d'), value: money(summary.commissionTaken30d) },
          { key: 'toppedUp30d', label: translate('summary.toppedUp30d'), value: money(summary.toppedUp30d) },
        ]
      : [],
    labels: {
      title: translate('title'), subtitle: translate('subtitle'), search: translate('search'), status: translate('status'),
      empty: translate('empty'), error: translate('error'), forbidden: translate('forbidden'),
    },
  };
};

export type AdminCreditsView = ReturnType<typeof useAdminCredits>;
