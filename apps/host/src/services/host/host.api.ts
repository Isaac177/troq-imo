import type { components } from '@mungini/api';
import type { SessionClient } from '@mungini/session';
import { apiErrorCode } from '@mungini/session';

export type DashboardFigures = components['schemas']['DashboardFigures'];
export type PropertyView = components['schemas']['PropertyView'];
export type HostMembership = components['schemas']['HostMembership'];
export type MemberView = components['schemas']['MemberView'];
export type InvitationView = components['schemas']['InvitationView'];
export type ApplicationView = components['schemas']['ApplicationView'];
export type DocumentView = components['schemas']['HostDocumentView'];
export type PermissionView = components['schemas']['PermissionView'];
export type PresetView = components['schemas']['PresetView'];

export type Paged<T> = { items: T[]; total: number; page: number; pageSize: number };

export type Result<T> = { data: T | null; error: string | null };

const fail = <T>(error: unknown): Result<T> => ({ data: null, error: apiErrorCode(error) });

export const hostApi = (client: SessionClient) => ({
  async dashboard(hostId: string, from: string, to: string): Promise<Result<DashboardFigures>> {
    const response = await client.api.GET('/api/host/dashboard', { params: { query: { hostId, from, to } } });
    if (response.error || !response.data) return fail(response.error);
    return { data: response.data.data, error: null };
  },
  async properties(hostId: string, page: number, pageSize: number, search: string): Promise<Result<Paged<PropertyView>>> {
    const response = await client.api.GET('/api/host/properties', {
      params: { query: { hostId, page, pageSize, ...(search ? { search } : {}) } },
    });
    if (response.error || !response.data) return fail(response.error);
    return { data: response.data.data, error: null };
  },
  async hosts(page: number, pageSize: number): Promise<Result<Paged<HostMembership>>> {
    const response = await client.api.GET('/api/host/hosts', { params: { query: { page, pageSize } } });
    if (response.error || !response.data) return fail(response.error);
    return { data: response.data.data, error: null };
  },
  async members(hostId: string, page: number, pageSize: number): Promise<Result<Paged<MemberView>>> {
    const response = await client.api.GET('/api/host/hosts/{hostId}/members', { params: { path: { hostId }, query: { page, pageSize } } });
    if (response.error || !response.data) return fail(response.error);
    return { data: response.data.data, error: null };
  },
  async setPermissions(hostId: string, userId: string, permissions: string[]): Promise<Result<MemberView>> {
    const response = await client.api.PUT('/api/host/hosts/{hostId}/members/{userId}/permissions', {
      params: { path: { hostId, userId } },
      body: { permissions },
    });
    if (response.error || !response.data) return fail(response.error);
    return { data: response.data.data, error: null };
  },
  async removeMember(hostId: string, userId: string): Promise<Result<null>> {
    const response = (await client.api.DELETE('/api/host/hosts/{hostId}/members/{userId}', { params: { path: { hostId, userId } } })) as { error?: unknown };
    if (response.error) return fail(response.error);
    return { data: null, error: null };
  },
  async invitations(hostId: string, page: number, pageSize: number): Promise<Result<Paged<InvitationView>>> {
    const response = await client.api.GET('/api/host/hosts/{hostId}/invitations', { params: { path: { hostId }, query: { page, pageSize } } });
    if (response.error || !response.data) return fail(response.error);
    return { data: response.data.data, error: null };
  },
  async invite(hostId: string, email: string, permissions: string[]): Promise<Result<InvitationView>> {
    const response = await client.api.POST('/api/host/hosts/{hostId}/invitations', { params: { path: { hostId } }, body: { email, permissions } });
    if (response.error || !response.data) return fail(response.error);
    return { data: response.data.data, error: null };
  },
  async revokeInvitation(hostId: string, invitationId: string): Promise<Result<null>> {
    const response = (await client.api.DELETE('/api/host/hosts/{hostId}/invitations/{invitationId}', { params: { path: { hostId, invitationId } } })) as { error?: unknown };
    if (response.error) return fail(response.error);
    return { data: null, error: null };
  },
  async catalog(): Promise<Result<{ permissions: PermissionView[]; presets: PresetView[] }>> {
    const response = await client.api.GET('/api/public/permission-catalog');
    if (!response.data) return { data: null, error: 'INTERNAL_ERROR' };
    return { data: response.data.data, error: null };
  },
  async myApplications(page: number, pageSize: number): Promise<Result<Paged<ApplicationView>>> {
    const response = await client.api.GET('/api/host/applications', { params: { query: { page, pageSize } } });
    if (response.error || !response.data) return fail(response.error);
    return { data: response.data.data, error: null };
  },
  async saveApplication(displayName: string, kind: string): Promise<Result<ApplicationView>> {
    const response = await client.api.POST('/api/host/applications', { body: { displayName, kind } });
    if (response.error || !response.data) return fail(response.error);
    return { data: response.data.data, error: null };
  },
  async submitApplication(applicationId: string): Promise<Result<ApplicationView>> {
    const response = await client.api.POST('/api/host/applications/{applicationId}/submit', { params: { path: { applicationId } } });
    if (response.error || !response.data) return fail(response.error);
    return { data: response.data.data, error: null };
  },
  async withdrawApplication(applicationId: string): Promise<Result<ApplicationView>> {
    const response = await client.api.POST('/api/host/applications/{applicationId}/withdraw', { params: { path: { applicationId } } });
    if (response.error || !response.data) return fail(response.error);
    return { data: response.data.data, error: null };
  },
  async addDocument(applicationId: string, input: { kind: string; fileName: string; mimeType: string; contentBase64: string }): Promise<Result<DocumentView>> {
    const response = await client.api.POST('/api/host/applications/{applicationId}/documents', { params: { path: { applicationId } }, body: input });
    if (response.error || !response.data) return fail(response.error);
    return { data: response.data.data, error: null };
  },
  async removeDocument(applicationId: string, documentId: string): Promise<Result<null>> {
    const response = (await client.api.DELETE('/api/host/applications/{applicationId}/documents/{documentId}', { params: { path: { applicationId, documentId } } })) as { error?: unknown };
    if (response.error) return fail(response.error);
    return { data: null, error: null };
  },
});
