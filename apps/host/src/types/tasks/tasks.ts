import type { components } from '@mungini/api';

export type HostTask = components['schemas']['TaskView'];

export type TaskPage = {
  items: HostTask[];
  total: number;
  page: number;
  pageSize: number;
};

export type TaskState = 'OPEN' | 'IN_PROGRESS' | 'DONE' | 'CANCELLED';

export type TaskType = 'CHECK_IN' | 'CHECK_OUT' | 'HOUSEKEEPING' | 'MAINTENANCE' | 'SERVICE_REQUEST';

export type TaskPriority = 'LOW' | 'NORMAL' | 'HIGH' | 'URGENT';

export type TaskAction = 'START' | 'COMPLETE' | 'CANCEL' | 'ASSIGN_ME' | 'UNASSIGN';

export type TaskActionItem = {
  action: TaskAction;
  label: string;
  run: () => void;
};

export type TaskPropertyOption = {
  value: string;
  label: string;
};

export type TaskResult<T> = {
  data: T | null;
  error: string | null;
};
