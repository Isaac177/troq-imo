import type { components } from '@mungini/api';

export const TASK_TYPES = ['CHECK_IN', 'CHECK_OUT', 'HOUSEKEEPING', 'MAINTENANCE', 'SERVICE_REQUEST'] as const;
export const TASK_STATES = ['OPEN', 'IN_PROGRESS', 'DONE', 'CANCELLED'] as const;
export const TASK_PRIORITIES = ['URGENT', 'HIGH', 'NORMAL', 'LOW'] as const;
export const TASK_ACTIONS = ['START', 'COMPLETE', 'CANCEL'] as const;

export type TaskAction = (typeof TASK_ACTIONS)[number];
export type AdminTask = components['schemas']['AdminTaskItem'];
export type AdminHost = components['schemas']['AdminHostView'];
export type AdminProperty = components['schemas']['AdminPropertyItem'];
export type CreateTaskBody = components['schemas']['CreateTaskInput'];
export type RequestResult<T> = { data: T | null; error: string | null };
export type PagedResult<T> = { items: T[]; total: number; page: number; pageSize: number };

export type TaskQuery = {
  state: string;
  taskType: string;
  priority: string;
  hostId: string;
  propertyId: string;
  assignee: string;
  search: string;
  page: number;
  pageSize: number;
};
