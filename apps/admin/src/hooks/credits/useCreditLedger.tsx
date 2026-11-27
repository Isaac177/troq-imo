import { useEffect, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { type TableColumn } from '@mungini/ui-web';
import { formatCdf, formatDateTime } from '@mungini/utils';
import { getCreditLedger } from '@/services/credits/credits.api';
import type { CreditLedgerRow } from '@/types/credits/credits';

const PAGE_SIZE = 10;
const ENTRY_TYPES = ['TOPUP', 'COMMISSION', 'REFUND', 'ADJUSTMENT'];

export const useCreditLedger = (hostId: string | null, reloadToken: number, timezone: string) => {
  const { t, i18n } = useTranslation('adminCredits');
  const translate = t as unknown as (key: string) => string;
  const { client } = useSession();
  const [rows, setRows] = useState<CreditLedgerRow[]>([]);
  const [total, setTotal] = useState(0);
  const [page, setPage] = useState(1);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    setRows([]);
    setTotal(0);
    setPage(1);
    setError(null);
  }, [hostId]);

  useEffect(() => {
    if (!hostId) return;
    let active = true;
    setLoading(true);
    setError(null);
    void getCreditLedger(client, hostId, page, PAGE_SIZE).then((result) => {
      if (!active) return;
      setLoading(false);
      if (result.error || !result.data) {
        setError(translate('ledger.error'));
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
  }, [client, hostId, page, reloadToken, translate]);

  const typeLabel = (value: string) => (ENTRY_TYPES.includes(value) ? translate(`entryTypes.${value}`) : value);
  const money = (value: string) => {
    const formatted = formatCdf(value, i18n.language);
    return Number(value) > 0 ? `+${formatted}` : formatted;
  };

  const columns: TableColumn<CreditLedgerRow>[] = [
    {
      key: 'type',
      header: translate('ledger.type'),
      cell: (row) => (
        <div className="min-w-0">
          <p className="truncate font-medium text-foreground">{typeLabel(row.entryType)}</p>
          {row.note || row.reservationReference ? <p className="truncate text-label text-muted-foreground">{row.note || row.reservationReference}</p> : null}
          <p className="text-label tabular-nums text-muted-foreground sm:hidden">{formatDateTime(row.createdAt, timezone)}</p>
        </div>
      ),
    },
    {
      key: 'amount',
      header: translate('ledger.amount'),
      align: 'right',
      cell: (row) => <span className="whitespace-nowrap tabular-nums">{money(row.amount)}</span>,
    },
    {
      key: 'balanceAfter',
      header: translate('ledger.balanceAfter'),
      align: 'right',
      hideBelow: 'sm',
      cell: (row) => <span className="whitespace-nowrap tabular-nums">{formatCdf(row.balanceAfter, i18n.language)}</span>,
    },
    {
      key: 'date',
      header: translate('ledger.date'),
      hideBelow: 'sm',
      cell: (row) => <span className="whitespace-nowrap tabular-nums">{formatDateTime(row.createdAt, timezone)}</span>,
    },
  ];

  return {
    rows, total, page, setPage, pageSize: PAGE_SIZE, loading, error, columns,
    labels: { title: translate('detail.ledger'), empty: translate('ledger.empty') },
  };
};

export type CreditLedgerView = ReturnType<typeof useCreditLedger>;
