import type { SessionClient } from '@mungini/session';
import { apiErrorCode } from '@mungini/session';
import type { HostTask, TaskAction, TaskPage, TaskPriority, TaskResult, TaskType } from '@/types/tasks/tasks';

export type TaskListQuery = {
  propertyId: string;
  state: string;
  taskType: string;
  page: number;
  pageSize: number;
};

export type TaskCreateInput = {
  propertyId: string;
  taskType: TaskType;
  priority: TaskPriority;
  checklist: string[];
};

const fail = <T>(error: unknown): TaskResult<T> => ({ data: null, error: apiErrorCode(error) });

export const listTasks = async (client: SessionClient, query: TaskListQuery): Promise<TaskResult<TaskPage>> => {
  const response = await client.api.GET('/api/host/tasks', {
    params: {
      query: {
        propertyId: query.propertyId,
        page: query.page,
        pageSize: query.pageSize,
        ...(query.state ? { state: query.state } : {}),
        ...(query.taskType ? { taskType: query.taskType } : {}),
      },
    },
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const createTask = async (client: SessionClient, input: TaskCreateInput): Promise<TaskResult<HostTask>> => {
  const response = await client.api.POST('/api/host/tasks', {
    body: {
      propertyId: input.propertyId,
      taskType: input.taskType,
      priority: input.priority,
      checklist: input.checklist,
    },
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const transitionTask = async (client: SessionClient, taskId: string, action: TaskAction, expectedVersion: number): Promise<TaskResult<HostTask>> => {
  const response = await client.api.POST('/api/host/tasks/{taskId}/transition', {
    params: { path: { taskId } },
    body: { action, expectedVersion },
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const listCities = (client: SessionClient) => client.api.GET('/api/public/cities');
