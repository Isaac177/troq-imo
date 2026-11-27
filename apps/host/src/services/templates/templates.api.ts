import type { components } from '@mungini/api';
import type { SessionClient } from '@mungini/session';
import { apiErrorCode } from '@mungini/session';
import type {
  RequestResult,
  Template,
  TemplateInput,
  TemplateListQuery,
  TemplateMedia,
  TemplateMediaInput,
  TemplateMediaUpdate,
  TemplatePage,
} from '@/types/templates/templates';

const accessibilityOf = (value: unknown): string[] => (
  Array.isArray(value) ? value.filter((item): item is string => typeof item === 'string') : []
);

const templateOf = (item: components['schemas']['TemplateView']): Template => ({ ...item, accessibility: accessibilityOf(item.accessibility) });

const fail = <T>(error: unknown): RequestResult<T> => ({ data: null, error: apiErrorCode(error) });

export const listTemplates = async (client: SessionClient, query: TemplateListQuery): Promise<RequestResult<TemplatePage>> => {
  const response = await client.api.GET('/api/host/accommodation-templates', {
    params: {
      query: {
        hostId: query.hostId,
        page: query.page,
        pageSize: query.pageSize,
        ...(query.search ? { search: query.search } : {}),
        ...(query.status && query.status !== 'ALL' ? { status: query.status } : {}),
        ...(query.categoryCode && query.categoryCode !== 'ALL' ? { categoryCode: query.categoryCode } : {}),
      },
    },
  });
  if (response.error || !response.data) return fail(response.error);
  return {
    data: {
      items: response.data.data.items.map(templateOf),
      total: response.data.data.total,
      page: response.data.data.page,
      pageSize: response.data.data.pageSize,
    },
    error: null,
  };
};

export const createTemplate = async (client: SessionClient, input: TemplateInput): Promise<RequestResult<Template>> => {
  const response = await client.api.POST('/api/host/accommodation-templates', { body: input });
  if (response.error || !response.data) return fail(response.error);
  return { data: templateOf(response.data.data), error: null };
};

export const updateTemplate = async (client: SessionClient, templateId: string, input: TemplateInput): Promise<RequestResult<Template>> => {
  const response = await client.api.PATCH('/api/host/accommodation-templates/{templateId}', {
    params: { path: { templateId } },
    body: input,
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: templateOf(response.data.data), error: null };
};

const mediaContentUrl = async (client: SessionClient, templateId: string, mediaId: string): Promise<string | null> => {
  try {
    const response = await client.api.GET('/api/host/accommodation-templates/{templateId}/media/{mediaId}/content', {
      params: { path: { templateId, mediaId } },
    });
    return response.data?.data.url ?? null;
  } catch {
    return null;
  }
};

export const listTemplateMedia = async (client: SessionClient, templateId: string): Promise<RequestResult<TemplateMedia[]>> => {
  const response = await client.api.GET('/api/host/accommodation-templates/{templateId}/media', {
    params: { path: { templateId } },
  });
  if (response.error || !response.data) return fail(response.error);
  const media = await Promise.all(response.data.data.map(async (item) => {
    if (item.status !== 'READY') return { ...item, url: null };
    return { ...item, url: await mediaContentUrl(client, templateId, item.id) };
  }));
  return { data: media, error: null };
};

export const addTemplateMedia = async (client: SessionClient, templateId: string, input: TemplateMediaInput): Promise<RequestResult<TemplateMedia>> => {
  const response = await client.api.POST('/api/host/accommodation-templates/{templateId}/media', {
    params: { path: { templateId } },
    body: { ...input, clientKey: crypto.randomUUID() },
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: { ...response.data.data, url: null }, error: null };
};

export const updateTemplateMedia = async (client: SessionClient, templateId: string, mediaId: string, input: TemplateMediaUpdate): Promise<RequestResult<TemplateMedia>> => {
  const response = await client.api.PATCH('/api/host/accommodation-templates/{templateId}/media/{mediaId}', {
    params: { path: { templateId, mediaId } },
    body: input,
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: { ...response.data.data, url: null }, error: null };
};

export const removeTemplateMedia = async (client: SessionClient, templateId: string, mediaId: string, expectedVersion: number): Promise<RequestResult<{ mediaId: string; status: string }>> => {
  const response = await client.api.DELETE('/api/host/accommodation-templates/{templateId}/media/{mediaId}', {
    params: { path: { templateId, mediaId } },
    body: { expectedVersion },
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};
