import type { SessionClient } from '@mungini/session';
import { apiErrorCode } from '@mungini/session';
import type { AdminHost, AdminProperty, AdminTask, CreateTaskBody, PagedResult, RequestResult, TaskAction, TaskQuery } from '@/types/tasks/tasks';

const fail = <T>(error: unknown): RequestResult<T> => ({ data: null, error: apiErrorCode(error) });

export const listTasks = async (client: SessionClient, query: TaskQuery): Promise<RequestResult<PagedResult<AdminTask>>> => {
  const response = await client.api.GET('/api/admin/tasks', {
    params: {
      query: {
        page: query.page,
        pageSize: query.pageSize,
        ...(query.state ? { state: query.state } : {}),
        ...(query.taskType ? { taskType: query.taskType } : {}),
        ...(query.priority ? { priority: query.priority } : {}),
        ...(query.hostId ? { hostId: query.hostId } : {}),
        ...(query.propertyId ? { propertyId: query.propertyId } : {}),
        ...(query.assignee ? { assignee: query.assignee } : {}),
        ...(query.search.trim() ? { search: query.search.trim() } : {}),
      },
    },
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const createTask = async (client: SessionClient, body: CreateTaskBody): Promise<RequestResult<AdminTask>> => {
  const response = await client.api.POST('/api/admin/tasks', { body });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const transitionTask = async (client: SessionClient, taskId: string, action: TaskAction, expectedVersion: number): Promise<RequestResult<AdminTask>> => {
  const response = await client.api.POST('/api/admin/tasks/{taskId}/transition', {
    params: { path: { taskId } },
    body: { action, expectedVersion },
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const listAdminHosts = async (client: SessionClient): Promise<RequestResult<PagedResult<AdminHost>>> => {
  const response = await client.api.GET('/api/admin/hosts', { params: { query: { page: 1, pageSize: 100 } } });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const listAdminProperties = async (client: SessionClient): Promise<RequestResult<PagedResult<AdminProperty>>> => {
  const response = await client.api.GET('/api/admin/properties', { params: { query: { page: 1, pageSize: 100 } } });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const listCities = (client: SessionClient) => client.api.GET('/api/public/cities');
