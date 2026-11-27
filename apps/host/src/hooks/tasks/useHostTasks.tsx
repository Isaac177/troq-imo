import { useCallback, useEffect, useMemo, useState } from 'react';
import { useSearchParams } from 'react-router-dom';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { textTones, type StatusTone } from '@mungini/theme/tones';
import { cn, StatusDot, type SelectOption, type TableColumn } from '@mungini/ui-web';
import { formatDateTime } from '@mungini/utils';
import TaskRowActions from '@/components/tasks/TaskRowActions';
import { useTaskCreate } from '@/hooks/tasks/useTaskCreate';
import { hostApi } from '@/services/host/host.api';
import { listCities, listTasks, transitionTask } from '@/services/tasks/tasks.api';
import type { HostTask, TaskAction, TaskActionItem, TaskPropertyOption, TaskState, TaskType } from '@/types/tasks/tasks';

const PAGE_SIZE = 20;
const STATES: TaskState[] = ['OPEN', 'IN_PROGRESS', 'DONE', 'CANCELLED'];
const TYPES: TaskType[] = ['CHECK_IN', 'CHECK_OUT', 'HOUSEKEEPING', 'SERVICE_REQUEST', 'MAINTENANCE'];
const OPERATIONS_PERMISSIONS = ['operations.checkin', 'operations.housekeeping', 'operations.maintenance'];
const TYPE_PERMISSIONS: Record<TaskType, string> = {
  CHECK_IN: 'operations.checkin',
  CHECK_OUT: 'operations.checkin',
  HOUSEKEEPING: 'operations.housekeeping',
  SERVICE_REQUEST: 'operations.housekeeping',
  MAINTENANCE: 'operations.maintenance',
};
const STATE_TONES: Record<string, StatusTone> = {
  OPEN: 'info',
  IN_PROGRESS: 'warning',
  DONE: 'success',
  CANCELLED: 'muted',
};
const PRIORITY_TONES: Record<string, string> = {
  LOW: 'text-muted-foreground',
  NORMAL: textTones.count,
  HIGH: textTones.attention,
  URGENT: textTones.problem,
};
const KNOWN_ERRORS = ['FORBIDDEN', 'TASK_TYPE_FORBIDDEN', 'PROPERTY_NOT_FOUND', 'TASK_STATE_UNKNOWN', 'TASK_TYPE_UNKNOWN', 'TASK_NOT_FOUND', 'TASK_ACTION_UNKNOWN', 'TASK_TRANSITION_INVALID', 'TASK_VERSION_CONFLICT'];

const actionsFor = (task: HostTask, userId: string): TaskAction[] => {
  if (task.state === 'DONE' || task.state === 'CANCELLED') return [];
  const actions: TaskAction[] = [];
  if (task.state === 'OPEN') actions.push('START');
  actions.push('COMPLETE', 'CANCEL');
  if (userId && task.assigneeUserId !== userId) actions.push('ASSIGN_ME');
  if (task.assigneeUserId) actions.push('UNASSIGN');
  return actions;
};

export const useHostTasks = () => {
  const { t, i18n } = useTranslation('hostTasks');
  const translate = t as unknown as (key: string, options?: Record<string, unknown>) => string;
  const { session, client, can, city } = useSession();
  const userId = session?.user.id ?? '';
  const hosts = (session?.hosts ?? []).filter((host) => OPERATIONS_PERMISSIONS.some((permission) => can(permission, host.hostId)));
  const [params, setParams] = useSearchParams();
  const requestedHost = params.get('host') ?? '';
  const hostId = hosts.some((host) => host.hostId === requestedHost) ? requestedHost : hosts[0]?.hostId ?? '';
  const allowedTypes = useMemo(() => TYPES.filter((type) => can(TYPE_PERMISSIONS[type], hostId)), [can, hostId]);
  const requestedState = params.get('state') ?? '';
  const state = STATES.includes(requestedState as TaskState) ? requestedState : '';
  const typeParam = params.get('type') ?? '';
  const taskType = allowedTypes.includes(typeParam as TaskType) ? typeParam : '';
  const pageValue = Number(params.get('page') ?? '1');
  const page = Number.isSafeInteger(pageValue) && pageValue > 0 ? pageValue : 1;
  const [propertyState, setPropertyState] = useState<{ hostId: string; options: TaskPropertyOption[] }>({ hostId: '', options: [] });
  const [propertiesLoading, setPropertiesLoading] = useState(false);
  const propertyOptions = propertyState.hostId === hostId ? propertyState.options : [];
  const requestedProperty = params.get('property') ?? '';
  const propertyId = propertyOptions.some((option) => option.value === requestedProperty) ? requestedProperty : propertyOptions[0]?.value ?? '';
  const [rows, setRows] = useState<HostTask[]>([]);
  const [total, setTotal] = useState(0);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');
  const [actionError, setActionError] = useState('');
  const [notice, setNotice] = useState('');
  const [forbidden, setForbidden] = useState(false);
  const [busy, setBusy] = useState<{ taskId: string; action: TaskAction } | null>(null);
  const [timezone, setTimezone] = useState('UTC');
  const [refreshToken, setRefreshToken] = useState(0);

  const update = useCallback((values: Record<string, string>) => {
    const next = new URLSearchParams(params);
    Object.entries(values).forEach(([key, value]) => (value ? next.set(key, value) : next.delete(key)));
    setNotice('');
    setActionError('');
    setParams(next, { replace: true });
  }, [params, setParams]);

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
    setError('');
    setForbidden(false);
    if (!hostId) {
      setPropertyState({ hostId: '', options: [] });
      return;
    }
    let active = true;
    setPropertiesLoading(true);
    void hostApi(client).properties(hostId, 1, 100, '').then((result) => {
      if (!active) return;
      setPropertiesLoading(false);
      if (result.error) {
        setPropertyState({ hostId, options: [] });
        if (result.error === 'FORBIDDEN') setForbidden(true);
        else setError(translate('error'));
        return;
      }
      const french = i18n.language.startsWith('fr');
      setPropertyState({
        hostId,
        options: (result.data?.items ?? []).map((property) => ({
          value: property.id,
          label: (french ? property.nameFr : property.nameEn) || property.nameEn || property.nameFr,
        })),
      });
    });
    return () => {
      active = false;
    };
  }, [client, hostId, i18n.language, translate]);

  useEffect(() => {
    if (!hostId || !propertyId) {
      setRows([]);
      setTotal(0);
      return;
    }
    let active = true;
    setLoading(true);
    setError('');
    void listTasks(client, { propertyId, state, taskType, page, pageSize: PAGE_SIZE }).then((result) => {
      if (!active) return;
      setLoading(false);
      if (result.error || !result.data) {
        setRows([]);
        setTotal(0);
        if (result.error === 'FORBIDDEN' || result.error === 'TASK_TYPE_FORBIDDEN') {
          setForbidden(true);
          return;
        }
        setError(translate('error'));
        return;
      }
      setForbidden(false);
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
  }, [client, hostId, propertyId, state, taskType, page, refreshToken, i18n.language, translate]);

  const message = (code: string | null) => (code && KNOWN_ERRORS.includes(code) ? translate(`errors.${code}`) : translate('errors.generic'));

  const run = async (task: HostTask, action: TaskAction) => {
    setBusy({ taskId: task.id, action });
    setActionError('');
    const result = await transitionTask(client, task.id, action, task.version);
    setBusy(null);
    if (result.error || !result.data) {
      setActionError(message(result.error));
      return;
    }
    setNotice(translate('updated'));
    setRefreshToken((value) => value + 1);
  };

  const stateLabel = (value: string) => (STATES.includes(value as TaskState) ? translate(`states.${value}`) : value);
  const typeLabel = (value: string) => (TYPES.includes(value as TaskType) ? translate(`types.${value}`) : value);
  const priorityLabel = (value: string) => translate(`priorities.${value}`);
  const assigneeLabel = (task: HostTask) => {
    const key = task.assigneeUserId ? (task.assigneeUserId === userId ? 'me' : 'other') : 'none';
    return translate(`assignee.${key}`);
  };
  const actionItems = (task: HostTask): TaskActionItem[] => actionsFor(task, userId).map((action) => ({
    action,
    label: translate(`actions.${action}`),
    run: () => void run(task, action),
  }));

  const columns: TableColumn<HostTask>[] = [
    {
      key: 'task',
      header: translate('colTask'),
      cell: (row) => <span className="font-medium text-foreground">{typeLabel(row.taskType)}</span>,
    },
    {
      key: 'state',
      header: translate('colState'),
      cell: (row) => <StatusDot tone={STATE_TONES[row.state] ?? 'neutral'} label={stateLabel(row.state)} />,
    },
    {
      key: 'priority',
      header: translate('colPriority'),
      cell: (row) => <span className={cn('font-medium', PRIORITY_TONES[row.priority] ?? 'text-muted-foreground')}>{priorityLabel(row.priority)}</span>,
    },
    {
      key: 'checklist',
      header: translate('colChecklist'),
      hideBelow: 'sm',
      cell: (row) => (row.checklist.length ? translate('checklistCount', { count: row.checklist.length }) : translate('noChecklist')),
    },
    {
      key: 'assignee',
      header: translate('colAssignee'),
      hideBelow: 'sm',
      cell: (row) => assigneeLabel(row),
    },
    {
      key: 'created',
      header: translate('colCreated'),
      hideBelow: 'sm',
      cell: (row) => <span className="whitespace-nowrap tabular-nums">{formatDateTime(row.createdAt, timezone)}</span>,
    },
    {
      key: 'actions',
      header: '',
      align: 'right',
      cell: (row) => <TaskRowActions actions={actionItems(row)} pending={busy?.taskId === row.id ? busy.action : null} />,
    },
  ];

  const create = useTaskCreate({
    propertyId,
    propertyName: propertyOptions.find((option) => option.value === propertyId)?.label ?? '',
    allowedTypes,
    onCreated: () => {
      setActionError('');
      setNotice(translate('created'));
      setRefreshToken((value) => value + 1);
    },
  });

  const stateOptions: SelectOption[] = [{ value: 'all', label: translate('allStates') }, ...STATES.map((value) => ({ value, label: translate(`states.${value}`) }))];
  const typeOptions: SelectOption[] = [{ value: 'all', label: translate('allTypes') }, ...allowedTypes.map((value) => ({ value, label: translate(`types.${value}`) }))];

  return {
    hosts: hosts.map((host) => ({ value: host.hostId, label: host.displayName })),
    hasHost: hosts.length > 0,
    hostId,
    setHostId: (id: string) => update({ host: id, property: '', type: '', page: '' }),
    propertyOptions,
    propertyId,
    propertiesLoading: propertiesLoading || (Boolean(hostId) && propertyState.hostId !== hostId),
    setPropertyId: (id: string) => update({ property: id, page: '' }),
    stateValue: state || 'all',
    setState: (value: string) => update({ state: value === 'all' ? '' : value, page: '' }),
    typeValue: taskType || 'all',
    setType: (value: string) => update({ type: value === 'all' ? '' : value, page: '' }),
    stateOptions,
    typeOptions,
    rows,
    total,
    page,
    pageSize: PAGE_SIZE,
    setPage: (value: number) => update({ page: value > 1 ? String(value) : '' }),
    loading,
    error,
    actionError,
    notice,
    forbidden,
    columns,
    canCreate: Boolean(propertyId) && allowedTypes.length > 0,
    create,
    labels: {
      title: translate('title'),
      subtitle: translate('subtitle'),
      host: translate('host'),
      property: translate('property'),
      chooseProperty: translate('chooseProperty'),
      state: translate('state'),
      type: translate('type'),
      newTask: translate('newTask'),
      empty: translate('empty'),
      forbidden: translate('forbidden'),
      noHost: translate('noHost'),
      noProperties: translate('noProperties'),
    },
  };
};
