import { useEffect, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import type { TableColumn } from '@mungini/ui-web';
import { formatCdf, formatDateTime } from '@mungini/utils';
import { getCreditLedger, getCreditSummary, listCities } from '@/services/credits/credits.api';
import type { CreditLedgerRow, CreditSummary } from '@/types/credits/credits';

const PAGE_SIZE = 20;
const TYPES = ['TOPUP', 'COMMISSION', 'REFUND', 'ADJUSTMENT'];

export const useHostCredits = () => {
  const { t, i18n } = useTranslation('hostCredits');
  const { t: th } = useTranslation('host');
  const translate = t as unknown as (key: string) => string;
  const translateHost = th as unknown as (key: string) => string;
  const { session, client, city } = useSession();
  const hosts = session?.hosts ?? [];
  const [hostId, setHostId] = useState(hosts[0]?.hostId ?? '');
  const [summary, setSummary] = useState<CreditSummary | null>(null);
  const [rows, setRows] = useState<CreditLedgerRow[]>([]);
  const [total, setTotal] = useState(0);
  const [page, setPage] = useState(1);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [timezone, setTimezone] = useState('UTC');

  useEffect(() => {
    if (!hosts.some((host) => host.hostId === hostId)) setHostId(hosts[0]?.hostId ?? '');
  }, [hosts, hostId]);

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
    if (!hostId) return;
    let active = true;
    setLoading(true);
    setError(null);
    void Promise.all([getCreditSummary(client, hostId), getCreditLedger(client, hostId, page, PAGE_SIZE)]).then(([summaryResult, ledgerResult]) => {
      if (!active) return;
      setLoading(false);
      if (summaryResult.error || ledgerResult.error || !summaryResult.data || !ledgerResult.data) {
        setError(translate('error'));
        setSummary(null);
        setRows([]);
        setTotal(0);
        return;
      }
      setSummary(summaryResult.data);
      setRows(ledgerResult.data.items);
      setTotal(ledgerResult.data.total);
    });
    return () => {
      active = false;
    };
  }, [client, hostId, page, translate]);

  const typeLabel = (value: string) => (TYPES.includes(value) ? translate(`types.${value}`) : value);
  const columns: TableColumn<CreditLedgerRow>[] = [
    {
      key: 'date',
      header: translate('date'),
      hideBelow: 'sm',
      cell: (row) => <span className="whitespace-nowrap tabular-nums">{formatDateTime(row.createdAt, timezone)}</span>,
    },
    {
      key: 'type',
      header: translate('type'),
      cell: (row) => (
        <div className="min-w-0">
          <p className="font-medium text-foreground">{typeLabel(row.entryType)}</p>
          <p className="truncate text-label text-muted-foreground">{row.note || row.reservationReference || row.paymentRef || ''}</p>
          <p className="text-label tabular-nums text-muted-foreground sm:hidden">{formatDateTime(row.createdAt, timezone)}</p>
        </div>
      ),
    },
    {
      key: 'amount',
      header: translate('amount'),
      align: 'right',
      cell: (row) => <span className="tabular-nums">{formatCdf(row.amount, i18n.language)}</span>,
    },
    {
      key: 'balanceAfter',
      header: translate('balanceAfter'),
      align: 'right',
      hideBelow: 'sm',
      cell: (row) => <span className="tabular-nums">{formatCdf(row.balanceAfter, i18n.language)}</span>,
    },
  ];

  const statusTone: 'danger' | 'warning' | 'success' = summary?.enabled === false ? 'danger' : summary?.low ? 'warning' : 'success';
  return {
    hosts,
    hasHost: hosts.length > 0,
    hostId,
    setHostId,
    summary,
    rows,
    total,
    page,
    setPage,
    pageSize: PAGE_SIZE,
    loading,
    error,
    columns,
    statusTone,
    balance: summary ? formatCdf(summary.balance, i18n.language) : '',
    rate: summary ? `${(summary.rateBps / 100).toFixed(2)}%` : '',
    threshold: summary ? formatCdf(summary.lowThreshold / 100, i18n.language) : '',
    status: summary ? (summary.enabled === false ? translate('disabled') : summary.low ? translate('low') : translate('enabled')) : '',
    labels: {
      title: translate('title'),
      subtitle: translate('subtitle'),
      balance: translate('balance'),
      commissionRate: translate('commissionRate'),
      status: translate('status'),
      lowThreshold: translate('lowThreshold'),
      ledger: translate('ledger'),
      empty: translate('empty'),
      error: translate('error'),
      loading: translate('loading'),
      noHost: translateHost('noHost'),
      noHostAction: translateHost('noHostAction'),
    },
  };
};
