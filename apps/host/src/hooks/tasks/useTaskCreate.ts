import { useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { createTask } from '@/services/tasks/tasks.api';
import type { TaskPriority, TaskType } from '@/types/tasks/tasks';

const PRIORITIES: TaskPriority[] = ['LOW', 'NORMAL', 'HIGH', 'URGENT'];
const KNOWN_ERRORS = [
  'FORBIDDEN',
  'PROPERTY_NOT_FOUND',
  'TASK_TYPE_UNKNOWN',
  'TASK_TYPE_FORBIDDEN',
  'TASK_PRIORITY_UNKNOWN',
  'TASK_PROPERTY_REQUIRED',
  'CHECKLIST_TOO_LONG',
  'CHECKLIST_ITEM_TOO_LONG',
  'UNIT_NOT_IN_PROPERTY',
  'ITEM_NOT_IN_PROPERTY',
];

type Options = {
  propertyId: string;
  propertyName: string;
  allowedTypes: TaskType[];
  onCreated: () => void;
};

export const useTaskCreate = ({ propertyId, propertyName, allowedTypes, onCreated }: Options) => {
  const { t } = useTranslation('hostTasks');
  const translate = t as unknown as (key: string, options?: Record<string, unknown>) => string;
  const { client } = useSession();
  const [open, setOpen] = useState(false);
  const [taskType, setTaskType] = useState('');
  const [priority, setPriority] = useState<TaskPriority>('NORMAL');
  const [checklist, setChecklist] = useState('');
  const [busy, setBusy] = useState(false);
  const [formError, setFormError] = useState('');

  const message = (code: string | null) => (code && KNOWN_ERRORS.includes(code) ? translate(`errors.${code}`) : translate('errors.generic'));

  const openCreate = () => {
    setTaskType(allowedTypes[0] ?? '');
    setPriority('NORMAL');
    setChecklist('');
    setFormError('');
    setOpen(true);
  };

  const onOpenChange = (next: boolean) => {
    if (!next && busy) return;
    if (!next) setOpen(false);
  };

  const close = () => {
    if (busy) return;
    setOpen(false);
  };

  const submit = async () => {
    if (!propertyId || !taskType) return;
    const items = checklist.split('\n').map((line) => line.trim()).filter((line) => line.length > 0);
    setBusy(true);
    setFormError('');
    const result = await createTask(client, { propertyId, taskType: taskType as TaskType, priority, checklist: items });
    setBusy(false);
    if (result.error || !result.data) {
      setFormError(message(result.error));
      return;
    }
    setOpen(false);
    onCreated();
  };

  return {
    open,
    taskType,
    setTaskType,
    priority,
    setPriority,
    checklist,
    setChecklist,
    busy,
    formError,
    propertyName,
    canSubmit: Boolean(propertyId) && Boolean(taskType) && !busy,
    openCreate,
    onOpenChange,
    close,
    submit,
    typeOptions: allowedTypes.map((value) => ({ value, label: translate(`types.${value}`) })),
    priorityOptions: PRIORITIES.map((value) => ({ value, label: translate(`priorities.${value}`) })),
    labels: {
      title: translate('create.title'),
      description: translate('create.description'),
      property: translate('create.property'),
      type: translate('create.type'),
      priority: translate('create.priority'),
      checklist: translate('create.checklist'),
      checklistHint: translate('create.checklistHint'),
      checklistPlaceholder: translate('create.checklistPlaceholder'),
      submit: translate('create.submit'),
      cancel: translate('create.cancel'),
      chooseProperty: translate('chooseProperty'),
    },
  };
};

export type TaskCreateView = ReturnType<typeof useTaskCreate>;
