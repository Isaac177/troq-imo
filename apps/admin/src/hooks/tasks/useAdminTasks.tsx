import { useEffect, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { textTones, type StatusTone } from '@mungini/theme/tones';
import { StatusDot, useDebouncedValue, type TableColumn } from '@mungini/ui-web';
import { formatDateTime } from '@mungini/utils';
import TaskActionsCell from '@/components/tasks/TaskActionsCell';
import { createTask, listAdminHosts, listAdminProperties, listCities, listTasks, transitionTask } from '@/services/tasks/tasks.api';
import { TASK_PRIORITIES, TASK_STATES, TASK_TYPES, type AdminHost, type AdminProperty, type AdminTask, type TaskAction } from '@/types/tasks/tasks';

const PAGE_SIZE = 20;
const KNOWN_ERRORS = ['FORBIDDEN', 'PERMISSION_MISSING', 'PROPERTY_NOT_FOUND', 'TASK_PROPERTY_REQUIRED', 'TASK_NOT_FOUND', 'TASK_VERSION_CONFLICT', 'TASK_TRANSITION_INVALID', 'PLATFORM_CANNOT_TAKE_TASK', 'TASK_TYPE_UNKNOWN', 'TASK_PRIORITY_UNKNOWN', 'TASK_ACTION_UNKNOWN', 'TASK_TYPE_FORBIDDEN', 'CHECKLIST_TOO_LONG', 'CHECKLIST_ITEM_TOO_LONG'];

const stateTones: Record<string, StatusTone> = { OPEN: 'warning', IN_PROGRESS: 'info', DONE: 'success', CANCELLED: 'muted' };
const priorityClasses: Record<string, string> = { URGENT: textTones.problem, HIGH: textTones.attention, NORMAL: 'text-foreground', LOW: 'text-muted-foreground' };

const actionsFor = (state: string): TaskAction[] => {
  if (state === 'OPEN') return ['START', 'COMPLETE', 'CANCEL'];
  if (state === 'IN_PROGRESS') return ['COMPLETE', 'CANCEL'];
  return [];
};

export const useAdminTasks = () => {
  const { t, i18n } = useTranslation('adminTasks');
  const translate = t as unknown as (key: string, options?: Record<string, unknown>) => string;
  const { client, city } = useSession();
  const [search, setSearch] = useState('');
  const term = useDebouncedValue(search, 250);
  const [state, setState] = useState('all');
  const [taskType, setTaskType] = useState('all');
  const [priority, setPriority] = useState('all');
  const [hostId, setHostId] = useState('all');
  const [propertyId, setPropertyId] = useState('all');
  const [assignee, setAssignee] = useState('all');
  const [page, setPage] = useState(1);
  const [rows, setRows] = useState<AdminTask[]>([]);
  const [total, setTotal] = useState(0);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [forbidden, setForbidden] = useState(false);
  const [notice, setNotice] = useState<string | null>(null);
  const [actionError, setActionError] = useState<string | null>(null);
  const [running, setRunning] = useState<{ taskId: string; action: TaskAction } | null>(null);
  const [refreshToken, setRefreshToken] = useState(0);
  const [timezone, setTimezone] = useState('UTC');
  const [hosts, setHosts] = useState<AdminHost[]>([]);
  const [properties, setProperties] = useState<AdminProperty[]>([]);
  const [createOpen, setCreateOpen] = useState(false);
  const [formHostId, setFormHostId] = useState('all');
  const [formPropertyId, setFormPropertyId] = useState('');
  const [formTaskType, setFormTaskType] = useState('HOUSEKEEPING');
  const [formPriority, setFormPriority] = useState('NORMAL');
  const [checklistText, setChecklistText] = useState('');
  const [formError, setFormError] = useState<string | null>(null);
  const [creating, setCreating] = useState(false);
  const french = i18n.language.startsWith('fr');

  const label = (group: 'states' | 'types' | 'priorities', value: string) => translate(`${group}.${value}`, { defaultValue: value });
  const message = (code: string | null) => (code && KNOWN_ERRORS.includes(code) ? translate(`errors.${code}`) : translate('errors.generic'));
  const actionLabels: Record<TaskAction, string> = { START: translate('actions.START'), COMPLETE: translate('actions.COMPLETE'), CANCEL: translate('actions.CANCEL') };
  const propertyName = (property: AdminProperty) => (french ? property.nameFr : property.nameEn) || property.nameEn || property.nameFr;
  const taskPropertyName = (task: AdminTask) => (french ? task.propertyNameFr : task.propertyNameEn) || task.propertyNameEn || task.propertyNameFr;
  const hostName = (host: AdminHost) => host.displayName;

  useEffect(() => {
    setPage(1);
  }, [term, state, taskType, priority, hostId, propertyId, assignee, city]);

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
    void Promise.all([listAdminHosts(client), listAdminProperties(client)]).then(([hostResult, propertyResult]) => {
      if (!active) return;
      if (hostResult.data) setHosts(hostResult.data.items);
      if (propertyResult.data) setProperties(propertyResult.data.items);
    }).catch(() => undefined);
    return () => {
      active = false;
    };
  }, [client, city]);

  useEffect(() => {
    let active = true;
    setLoading(true);
    setError(null);
    setForbidden(false);
    void listTasks(client, { state: state === 'all' ? '' : state, taskType: taskType === 'all' ? '' : taskType, priority: priority === 'all' ? '' : priority, hostId: hostId === 'all' ? '' : hostId, propertyId: propertyId === 'all' ? '' : propertyId, assignee: assignee === 'all' ? '' : assignee, search: term, page, pageSize: PAGE_SIZE }).then((result) => {
      if (!active) return;
      setLoading(false);
      if (result.error || !result.data) {
        const denied = result.error === 'PERMISSION_MISSING' || result.error === 'FORBIDDEN';
        setForbidden(denied);
        setError(denied ? null : translate('errors.generic'));
        setRows([]);
        setTotal(0);
        return;
      }
      setRows(result.data.items);
      setTotal(result.data.total);
    }).catch(() => {
      if (!active) return;
      setLoading(false);
      setError(translate('errors.generic'));
      setRows([]);
      setTotal(0);
    });
    return () => {
      active = false;
    };
  }, [client, city, state, taskType, priority, hostId, propertyId, assignee, term, page, refreshToken, translate]);

  useEffect(() => {
    if (!loading && !error && !forbidden && rows.length === 0 && page > 1) setPage((value) => value - 1);
  }, [loading, error, forbidden, rows.length, page]);

  const changeHost = (value: string) => {
    setHostId(value);
    setPropertyId('all');
  };

  const applyAction = async (task: AdminTask, action: TaskAction) => {
    if (running) return;
    setRunning({ taskId: task.id, action });
    setActionError(null);
    setNotice(null);
    try {
      const result = await transitionTask(client, task.id, action, task.version);
      if (result.error || !result.data) {
        setActionError(message(result.error));
        return;
      }
      setNotice(translate('updated'));
      setRefreshToken((value) => value + 1);
    } catch {
      setActionError(message(null));
    } finally {
      setRunning(null);
    }
  };

  const openCreate = () => {
    setFormError(null);
    setCreateOpen(true);
  };

  const resetForm = () => {
    setFormHostId('all');
    setFormPropertyId('');
    setFormTaskType('HOUSEKEEPING');
    setFormPriority('NORMAL');
    setChecklistText('');
  };

  const closeCreate = () => {
    resetForm();
    setFormError(null);
    setCreateOpen(false);
  };

  const changeFormHost = (value: string) => {
    setFormHostId(value);
    setFormPropertyId('');
  };

  const submitCreate = async () => {
    if (!formPropertyId) {
      setFormError(translate('errors.PROPERTY_REQUIRED'));
      return;
    }
    setCreating(true);
    setFormError(null);
    try {
      const result = await createTask(client, {
        propertyId: formPropertyId,
        taskType: formTaskType,
        priority: formPriority,
        checklist: checklistText.split('\n').map((line) => line.trim()).filter(Boolean),
      });
      if (result.error || !result.data) {
        setFormError(message(result.error));
        return;
      }
      setCreateOpen(false);
      resetForm();
      setNotice(translate('created'));
      setRefreshToken((value) => value + 1);
    } catch {
      setFormError(message(null));
    } finally {
      setCreating(false);
    }
  };

  const propertiesForHost = (value: string) => properties.filter((property) => value === 'all' || property.hostId === value);
  const propertyOptions = properties.filter((property) => hostId === 'all' || property.hostId === hostId);

  const columns: TableColumn<AdminTask>[] = [
    {
      key: 'property',
      header: translate('columns.property'),
      cell: (row) => (
        <div className="min-w-0">
          <p className="truncate font-medium text-foreground">{taskPropertyName(row)}</p>
          <p className="truncate text-label text-muted-foreground">{row.operatorName}</p>
        </div>
      ),
    },
    { key: 'taskType', header: translate('columns.taskType'), hideBelow: 'sm', cell: (row) => label('types', row.taskType) },
    {
      key: 'reference',
      header: translate('columns.reference'),
      hideBelow: 'sm',
      cell: (row) => <span className="tabular-nums">{row.unitCode || row.reservationReference || '—'}</span>,
    },
    { key: 'state', header: translate('columns.state'), cell: (row) => <StatusDot tone={stateTones[row.state] ?? 'neutral'} label={label('states', row.state)} /> },
    { key: 'priority', header: translate('columns.priority'), cell: (row) => <span className={priorityClasses[row.priority] ?? 'text-foreground'}>{label('priorities', row.priority)}</span> },
    {
      key: 'assignee',
      header: translate('columns.assignee'),
      hideBelow: 'sm',
      cell: (row) => <span className={row.assigneeUserId ? textTones.people : 'text-muted-foreground'}>{row.assigneeUserId ? translate('assigned') : translate('unassigned')}</span>,
    },
    {
      key: 'createdAt',
      header: translate('columns.created'),
      hideBelow: 'sm',
      cell: (row) => <span className="whitespace-nowrap tabular-nums">{formatDateTime(row.createdAt, timezone)}</span>,
    },
    {
      key: 'actions',
      header: translate('columns.actions'),
      headerHidden: true,
      align: 'right',
      cell: (row) => <TaskActionsCell actions={actionsFor(row.state)} taskId={row.id} running={running} labels={actionLabels} onAction={(action) => void applyAction(row, action)} />,
    },
  ];

  return {
    search, setSearch, state, setState, taskType, setTaskType, priority, setPriority, hostId, changeHost, propertyId, setPropertyId, assignee, setAssignee,
    page, setPage, pageSize: PAGE_SIZE, rows, total, loading, error, forbidden, notice, actionError, columns,
    openCreate,
    hostOptions: [{ value: 'all', label: translate('allHosts') }, ...hosts.map((host) => ({ value: host.id, label: hostName(host) }))],
    propertyOptions: [{ value: 'all', label: translate('allProperties') }, ...propertyOptions.map((property) => ({ value: property.id, label: propertyName(property) }))],
    stateOptions: [{ value: 'all', label: translate('allStates') }, ...TASK_STATES.map((value) => ({ value, label: label('states', value) }))],
    typeOptions: [{ value: 'all', label: translate('allTypes') }, ...TASK_TYPES.map((value) => ({ value, label: label('types', value) }))],
    priorityOptions: [{ value: 'all', label: translate('allPriorities') }, ...TASK_PRIORITIES.map((value) => ({ value, label: label('priorities', value) }))],
    assigneeOptions: [{ value: 'all', label: translate('allAssignees') }, { value: 'none', label: translate('unassigned') }],
    labels: {
      title: translate('title'), subtitle: translate('subtitle'), newTask: translate('newTask'), search: translate('search'),
      host: translate('columns.host'), property: translate('columns.property'), state: translate('columns.state'), taskType: translate('columns.taskType'),
      priority: translate('columns.priority'), assignee: translate('columns.assignee'), empty: translate('empty'), emptyHint: translate('emptyHint'),
      forbidden: translate('forbidden'), forbiddenHint: translate('forbiddenHint'),
    },
    create: {
      open: createOpen,
      openDialog: openCreate,
      closeDialog: closeCreate,
      hostId: formHostId,
      setHostId: changeFormHost,
      propertyId: formPropertyId,
      setPropertyId: setFormPropertyId,
      taskType: formTaskType,
      setTaskType: setFormTaskType,
      priority: formPriority,
      setPriority: setFormPriority,
      checklistText,
      setChecklistText,
      error: formError,
      busy: creating,
      submit: () => void submitCreate(),
      hostOptions: [{ value: 'all', label: translate('create.allHosts') }, ...hosts.map((host) => ({ value: host.id, label: hostName(host) }))],
      propertyOptions: propertiesForHost(formHostId).map((property) => ({ value: property.id, label: propertyName(property) })),
      typeOptions: TASK_TYPES.map((value) => ({ value, label: label('types', value) })),
      priorityOptions: TASK_PRIORITIES.map((value) => ({ value, label: label('priorities', value) })),
      labels: {
        title: translate('create.title'), description: translate('create.description'), host: translate('create.host'),
        property: translate('create.property'), propertyPlaceholder: translate('create.propertyPlaceholder'), type: translate('create.type'),
        priority: translate('create.priority'), checklist: translate('create.checklist'), checklistPlaceholder: translate('create.checklistPlaceholder'),
        cancel: translate('create.cancel'), submit: translate('create.submit'),
      },
    },
  };
};

export type AdminTasksView = ReturnType<typeof useAdminTasks>;
