import { useCallback, useEffect, useState } from 'react';
import { useSearchParams } from 'react-router-dom';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { textTones } from '@mungini/theme/tones';
import { StatusDot, cn, type SelectOption, type TableColumn } from '@mungini/ui-web';
import { formatDateTime, formatDay } from '@mungini/utils';
import { useCityTimezone } from '@/hooks/cases/useCityTimezone';
import { listCases } from '@/services/cases/cases.api';
import { CASE_STATES, CASE_STATE_TONES, type CaseState, type HostCase } from '@/types/cases/cases';

const PAGE_SIZE = 20;
const NAMESPACE = 'hostCases';
const KNOWN_LIST_ERRORS = ['FORBIDDEN', 'HOST_NOT_FOUND', 'HOST_REQUIRED'];

export const useHostCases = () => {
  const { t } = useTranslation();
  const translateRaw = t as unknown as (key: string, options?: Record<string, unknown>) => string;
  const translate = useCallback((key: string, options?: Record<string, unknown>) => translateRaw(key, { ...options, ns: NAMESPACE }), [translateRaw]);
  const { session, client, can } = useSession();
  const hosts = (session?.hosts ?? []).filter((host) => can('reservations.read', host.hostId));
  const [params, setParams] = useSearchParams();
  const requestedHost = params.get('host') ?? '';
  const hostId = hosts.some((host) => host.hostId === requestedHost) ? requestedHost : hosts[0]?.hostId ?? '';
  const requestedState = params.get('state') ?? '';
  const state = CASE_STATES.includes(requestedState as CaseState) ? requestedState : '';
  const pageValue = Number(params.get('page') ?? '1');
  const page = Number.isSafeInteger(pageValue) && pageValue > 0 ? pageValue : 1;
  const selectedParam = params.get('case') ?? '';
  const selected = selectedParam || null;
  const canWrite = Boolean(hostId) && can('cases.write', hostId);
  const timezone = useCityTimezone();
  const [rows, setRows] = useState<HostCase[]>([]);
  const [total, setTotal] = useState(0);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');
  const [forbidden, setForbidden] = useState(false);
  const [notice, setNotice] = useState('');
  const [refreshToken, setRefreshToken] = useState(0);

  const update = useCallback((values: Record<string, string>) => {
    const next = new URLSearchParams(params);
    Object.entries(values).forEach(([key, value]) => (value ? next.set(key, value) : next.delete(key)));
    setNotice('');
    setParams(next, { replace: true });
  }, [params, setParams]);

  useEffect(() => {
    setError('');
    setForbidden(false);
    if (!hostId) {
      setRows([]);
      setTotal(0);
      return;
    }
    let active = true;
    setLoading(true);
    void listCases(client, { hostId, state, page, pageSize: PAGE_SIZE }).then((result) => {
      if (!active) return;
      setLoading(false);
      if (result.error || !result.data) {
        setRows([]);
        setTotal(0);
        if (result.error === 'FORBIDDEN') {
          setForbidden(true);
          return;
        }
        setError(KNOWN_LIST_ERRORS.includes(result.error ?? '') ? translate(`errors.${result.error}`) : translate('error'));
        return;
      }
      setRows(result.data.items);
      setTotal(result.data.total);
    }).catch(() => {
      if (!active) return;
      setLoading(false);
      setError(translate('error'));
    });
    return () => {
      active = false;
    };
  }, [client, hostId, state, page, refreshToken, translate]);

  const stateLabel = (value: string) => translate(`states.${value}`);
  const typeLabel = (value: string) => translate(`types.${value}`);
  const deadlineClass = cn('whitespace-nowrap tabular-nums', textTones.attention);

  const columns: TableColumn<HostCase>[] = [
    {
      key: 'case',
      header: translate('colCase'),
      cell: (row) => (
        <div className="min-w-0">
          <p className="truncate font-medium text-foreground">{typeLabel(row.caseType)}</p>
          <p className="truncate text-label text-muted-foreground">{row.reservationReference ?? translate('noReservation')}</p>
          <p className="mt-1 sm:hidden"><StatusDot tone={CASE_STATE_TONES[row.state] ?? 'neutral'} label={stateLabel(row.state)} /></p>
          <p className={cn('text-label sm:hidden', textTones.attention)}>{formatDateTime(row.deadlineAt, timezone)}</p>
        </div>
      ),
    },
    {
      key: 'reason',
      header: translate('reason'),
      hideBelow: 'sm',
      cell: (row) => <p className="line-clamp-2 max-w-[320px] text-foreground">{row.reason}</p>,
    },
    {
      key: 'deadline',
      header: translate('deadline'),
      hideBelow: 'sm',
      cell: (row) => <span className={deadlineClass}>{formatDateTime(row.deadlineAt, timezone)}</span>,
    },
    {
      key: 'state',
      header: translate('state'),
      hideBelow: 'sm',
      cell: (row) => <StatusDot tone={CASE_STATE_TONES[row.state] ?? 'neutral'} label={stateLabel(row.state)} />,
    },
    {
      key: 'opened',
      header: translate('opened'),
      hideBelow: 'sm',
      cell: (row) => <span className="whitespace-nowrap tabular-nums">{formatDay(row.createdAt, timezone)}</span>,
    },
  ];

  const stateOptions: SelectOption[] = [
    { value: 'all', label: translate('allStates') },
    ...CASE_STATES.map((value) => ({ value, label: stateLabel(value) })),
  ];

  return {
    hosts: hosts.map((host) => ({ value: host.hostId, label: host.displayName })),
    hasHost: hosts.length > 0,
    hostId,
    setHostId: (id: string) => update({ host: id, page: '', case: '' }),
    stateValue: state || 'all',
    setState: (value: string) => update({ state: value === 'all' ? '' : value, page: '', case: '' }),
    stateOptions,
    canWrite,
    rows,
    total,
    page,
    pageSize: PAGE_SIZE,
    setPage: (value: number) => update({ page: value > 1 ? String(value) : '' }),
    loading,
    error,
    forbidden,
    notice,
    columns,
    selected,
    setSelected: (id: string) => update({ case: id }),
    closeDetail: () => update({ case: '' }),
    refresh: () => setRefreshToken((value) => value + 1),
    onCaseOpened: () => {
      setNotice(translate('caseOpened'));
      setRefreshToken((value) => value + 1);
    },
    labels: {
      title: translate('title'),
      subtitle: translate('subtitle'),
      host: translate('host'),
      state: translate('state'),
      openCase: translate('openCase'),
      empty: translate('empty'),
      error: translate('error'),
      forbidden: translate('forbidden'),
      loading: translate('loading'),
      noHost: translate('noHost'),
      noHostAction: translate('noHostAction'),
    },
  };
};

export type HostCasesView = ReturnType<typeof useHostCases>;
