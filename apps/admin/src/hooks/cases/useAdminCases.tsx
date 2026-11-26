import { useEffect, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { useDebouncedValue, type TableColumn } from '@mungini/ui-web';
import { formatCdf, formatDateTime } from '@mungini/utils';
import { listCities, listCases } from '@/services/cases/cases.api';
import type { AdminCase } from '@/types/cases/cases';

const PAGE_SIZE = 20;

export const useAdminCases = () => {
  const { t, i18n } = useTranslation('adminCases');
  const translate = t as unknown as (key: string) => string;
  const { client, city } = useSession();
  const [search, setSearch] = useState('');
  const term = useDebouncedValue(search, 250);
  const [state, setState] = useState('all');
  const [caseType, setCaseType] = useState('all');
  const [overdue, setOverdue] = useState(false);
  const [page, setPage] = useState(1);
  const [rows, setRows] = useState<AdminCase[]>([]);
  const [total, setTotal] = useState(0);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [notice, setNotice] = useState<string | null>(null);
  const [selected, setSelected] = useState<string | null>(null);
  const [timezone, setTimezone] = useState('UTC');
  const [refreshToken, setRefreshToken] = useState(0);

  useEffect(() => {
    setPage(1);
  }, [term, state, caseType, overdue, city]);

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
    void listCases(client, { state: state === 'all' ? '' : state, caseType: caseType === 'all' ? '' : caseType, overdue, search: term, page, pageSize: PAGE_SIZE }).then((result) => {
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
  }, [client, city, state, caseType, overdue, term, page, refreshToken, translate]);

  const columns: TableColumn<AdminCase>[] = [
    {
      key: 'case',
      header: translate('caseType'),
      cell: (row) => (
        <div className="min-w-0">
          <p className="truncate font-medium text-foreground">{translate(`types.${row.caseType}`)}</p>
          <p className="truncate text-label text-muted-foreground">{row.operatorName}{row.reservationReference ? ` · ${row.reservationReference}` : ''}</p>
          <p className="truncate text-label text-muted-foreground sm:hidden">{translate(`states.${row.state}`)}</p>
        </div>
      ),
    },
    { key: 'state', header: translate('state'), hideBelow: 'sm', cell: (row) => translate(`states.${row.state}`) },
    { key: 'deadline', header: translate('deadline'), hideBelow: 'sm', cell: (row) => <span className={row.isOverdue ? 'tabular-nums text-destructive' : 'tabular-nums'}>{formatDateTime(row.deadlineAt, timezone)}</span> },
    { key: 'claim', header: translate('claimAmount'), align: 'right', hideBelow: 'sm', cell: (row) => <span className="tabular-nums">{row.claimAmount ? formatCdf(row.claimAmount, i18n.language) : '—'}</span> },
  ];

  return {
    search, setSearch, state, setState, caseType, setCaseType, overdue, setOverdue,
    page, setPage, pageSize: PAGE_SIZE, rows, total, loading, error, notice, columns,
    selected, setSelected,
    onChanged: () => setRefreshToken((value) => value + 1),
    stateOptions: [
      { value: 'all', label: translate('allStates') },
      ...['OPEN', 'AWAITING_RESPONSE', 'RESOLVED', 'CLOSED'].map((value) => ({ value, label: translate(`states.${value}`) })),
    ],
    typeOptions: [
      { value: 'all', label: translate('allTypes') },
      ...['GUEST_COMPLAINT', 'OPERATOR_COMPLAINT', 'DAMAGE_CLAIM', 'REFUND_DISPUTE', 'NO_SHOW_DISPUTE', 'GENERAL'].map((value) => ({ value, label: translate(`types.${value}`) })),
    ],
    labels: {
      title: translate('title'), subtitle: translate('subtitle'), search: translate('search'), state: translate('state'),
      caseType: translate('caseType'), overdueOnly: translate('overdueOnly'), empty: translate('empty'), error: translate('error'),
    },
  };
};
