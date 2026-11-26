import type { components } from '@mungini/api';
import type { SessionClient } from '@mungini/session';
import { apiErrorCode } from '@mungini/session';

export type AdminApplicationView = components['schemas']['AdminApplicationView'];
export type AdminHostView = components['schemas']['AdminHostView'];
export type ApplicationView = components['schemas']['ApplicationView'];
export type HostDocumentView = components['schemas']['HostDocumentView'];
export type GuestItem = components['schemas']['GuestItem'];
export type DashboardFigures = components['schemas']['DashboardFigures'];
export type HostFigures = components['schemas']['HostFigures'];
export type HostReferenceView = components['schemas']['HostReferenceView'];

export type Paged<T> = { items: T[]; total: number; page: number; pageSize: number };
export type Result<T> = { data: T | null; error: string | null };

const fail = <T>(error: unknown): Result<T> => ({ data: null, error: apiErrorCode(error) });

export const adminApi = (client: SessionClient) => ({
  async dashboard(from: string, to: string): Promise<Result<components['schemas']['DashboardSummary']>> {
    const response = await client.api.GET('/api/admin/dashboard', { params: { query: { from, to } } });
    if (response.error || !response.data) return fail(response.error);
    return { data: response.data.data, error: null };
  },
  async hostFigures(from: string, to: string, page: number, pageSize: number): Promise<Result<Paged<HostFigures>>> {
    const response = await client.api.GET('/api/admin/dashboard/hosts', { params: { query: { from, to, page, pageSize } } });
    if (response.error || !response.data) return fail(response.error);
    return { data: response.data.data, error: null };
  },
  async applications(status: string, search: string, page: number, pageSize: number): Promise<Result<Paged<AdminApplicationView>>> {
    const response = await client.api.GET('/api/admin/host-applications', {
      params: { query: { page, pageSize, ...(status ? { status } : {}), ...(search ? { search } : {}) } },
    });
    if (response.error || !response.data) return fail(response.error);
    return { data: response.data.data, error: null };
  },
  async application(applicationId: string): Promise<Result<ApplicationView>> {
    const response = await client.api.GET('/api/admin/host-applications/{applicationId}', { params: { path: { applicationId } } });
    if (response.error || !response.data) return fail(response.error);
    return { data: response.data.data, error: null };
  },
  async decide(applicationId: string, decision: string, reasonCode: string): Promise<Result<ApplicationView>> {
    const response = await client.api.POST('/api/admin/host-applications/{applicationId}/decision', {
      params: { path: { applicationId } },
      body: { decision, reasonCode },
    });
    if (response.error || !response.data) return fail(response.error);
    return { data: response.data.data, error: null };
  },
  async reviewDocument(applicationId: string, documentId: string, status: string, reasonCode: string | null): Promise<Result<HostDocumentView>> {
    const response = await client.api.POST('/api/admin/host-applications/{applicationId}/documents/{documentId}/review', {
      params: { path: { applicationId, documentId } },
      body: { status, ...(reasonCode ? { reasonCode } : {}) },
    });
    if (response.error || !response.data) return fail(response.error);
    return { data: response.data.data, error: null };
  },
  async hosts(status: string, search: string, page: number, pageSize: number): Promise<Result<Paged<AdminHostView>>> {
    const response = await client.api.GET('/api/admin/hosts', {
      params: { query: { page, pageSize, ...(status ? { status } : {}), ...(search ? { search } : {}) } },
    });
    if (response.error || !response.data) return fail(response.error);
    return { data: response.data.data, error: null };
  },
  async setHostStatus(hostId: string, action: string, reasonCode: string): Promise<Result<components['schemas']['HostStatusView']>> {
    const response = await client.api.POST('/api/admin/hosts/{hostId}/status', { params: { path: { hostId } }, body: { action, reasonCode } });
    if (response.error || !response.data) return fail(response.error);
    return { data: response.data.data, error: null };
  },
  async guests(search: string, flag: string, page: number, pageSize: number): Promise<Result<Paged<GuestItem>>> {
    const response = await client.api.GET('/api/admin/guests', {
      params: { query: { page, pageSize, ...(flag ? { flag } : {}), ...(search ? { search } : {}) } },
    });
    if (response.error || !response.data) return fail(response.error);
    return { data: response.data.data, error: null };
  },
  async guest(userId: string): Promise<Result<GuestItem>> {
    const response = await client.api.GET('/api/admin/guests/{userId}', { params: { path: { userId } } });
    if (response.error || !response.data) return fail(response.error);
    return { data: response.data.data, error: null };
  },
  async reference(): Promise<Result<HostReferenceView>> {
    const response = await client.api.GET('/api/public/host-reference');
    if (!response.data) return { data: null, error: 'INTERNAL_ERROR' };
    return { data: response.data.data, error: null };
  },
});
