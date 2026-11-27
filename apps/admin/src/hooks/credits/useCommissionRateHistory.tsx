import { useEffect, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { type TableColumn } from '@mungini/ui-web';
import { formatDateTime } from '@mungini/utils';
import { getCommissionRateHistory } from '@/services/credits/credits.api';
import type { CommissionRateRow } from '@/types/credits/credits';

const PAGE_SIZE = 10;

export const useCommissionRateHistory = (hostId: string | null, reloadToken: number, timezone: string) => {
  const { t, i18n } = useTranslation('adminCredits');
  const translate = t as unknown as (key: string) => string;
  const { client } = useSession();
  const [rows, setRows] = useState<CommissionRateRow[]>([]);
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
    void getCommissionRateHistory(client, hostId, page, PAGE_SIZE).then((result) => {
      if (!active) return;
      setLoading(false);
      if (result.error || !result.data) {
        setError(translate('rateHistory.error'));
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

  const percent = (bps: number) => new Intl.NumberFormat(i18n.language, { style: 'percent', minimumFractionDigits: 0, maximumFractionDigits: 2 }).format(bps / 10000);
  const reasonLabel = (value: string) => {
    const key = `reasons.${value}`;
    const label = translate(key);
    return label === key ? value : label;
  };

  const columns: TableColumn<CommissionRateRow>[] = [
    { key: 'effectiveFrom', header: translate('rateHistory.date'), cell: (row) => <span className="whitespace-nowrap tabular-nums">{formatDateTime(row.effectiveFrom, timezone)}</span> },
    { key: 'rate', header: translate('rateHistory.rate'), align: 'right', cell: (row) => <span className="tabular-nums">{percent(row.rateBps)}</span> },
    { key: 'reason', header: translate('rateHistory.reason'), hideBelow: 'sm', cell: (row) => reasonLabel(row.reasonCode) },
    { key: 'note', header: translate('rateHistory.note'), hideBelow: 'sm', cell: (row) => <span className="text-muted-foreground">{row.note || '—'}</span> },
  ];

  return {
    rows, total, page, setPage, pageSize: PAGE_SIZE, loading, error, columns,
    labels: { title: translate('detail.rateHistory'), empty: translate('rateHistory.empty') },
  };
};

export type CommissionRateHistoryView = ReturnType<typeof useCommissionRateHistory>;
