import type { components } from '@mungini/api';
import type { SessionClient } from '@mungini/session';
import { apiErrorCode } from '@mungini/session';

export type PropertyView = components['schemas']['PropertyView'];
export type UnitType = components['schemas']['Envelope_Page_UnitTypeView']['data']['items'][number];
export type Category = components['schemas']['CategoryView'];
export type PropertyMedia = { id: string; ownerId: string; status: string; sortOrder: number; isCover: boolean; altEn: string; altFr: string; fileName: string; mimeType: string; sizeBytes: number; rejectionReason: string | null; version: number; url: string | null };
export type RevisionSummary = components['schemas']['RevisionSummary'];
export type Revision = components['schemas']['RevisionView'];
export type ListingContent = components['schemas']['ListingContent'];
export type Amenity = components['schemas']['Amenity'];
export type RequestResult<T> = { data: T | null; error: string | null; blockers: string[] };

const fail = <T>(error: unknown): RequestResult<T> => {
  const params = (error as { error?: { params?: { blockers?: string[] } } })?.error?.params;
  return { data: null, error: apiErrorCode(error), blockers: params?.blockers ?? [] };
};

export const createProperty = async (client: SessionClient, input: { hostId: string; nameEn: string; nameFr: string; timezone: string; addressLine: string; descriptionEn: string; descriptionFr: string }): Promise<RequestResult<PropertyView>> => {
  const response = await client.api.POST('/api/host/properties', { body: { ...input, idempotencyKey: crypto.randomUUID() } });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null, blockers: [] };
};

export const propertyLifecycle = async (client: SessionClient, propertyId: string, action: 'PUBLISH' | 'PAUSE' | 'ARCHIVE', expectedVersion: number): Promise<RequestResult<PropertyView>> => {
  const response = await client.api.POST('/api/host/properties/{propertyId}/lifecycle', {
    params: { path: { propertyId } },
    body: { action, expectedVersion },
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null, blockers: [] };
};

export const listCities = (client: SessionClient) => client.api.GET('/api/public/cities');

export const listCategories = async (client: SessionClient): Promise<RequestResult<Category[]>> => {
  const response = await client.api.GET('/api/public/accommodation-categories');
  if (!response.data) return { data: null, error: 'INTERNAL_ERROR', blockers: [] };
  return { data: response.data.data.filter((category) => category.active), error: null, blockers: [] };
};

export const listUnitTypes = async (client: SessionClient, propertyId: string): Promise<RequestResult<UnitType[]>> => {
  const response = await client.api.GET('/api/host/properties/{propertyId}/unit-types', { params: { path: { propertyId }, query: { page: 1, pageSize: 100 } } });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data.items, error: null, blockers: [] };
};

export type UnitTypeBody = { nameEn: string; nameFr: string; categoryCode: string; inventoryModel: string; maxAdults: number; maxChildren: number; bedroomCount: number; bathroomCount: number; bedCount: number; sellableQuantity: number; descriptionEn: string; descriptionFr: string };

export const createUnitType = async (client: SessionClient, propertyId: string, body: UnitTypeBody): Promise<RequestResult<UnitType>> => {
  const response = await client.api.POST('/api/host/properties/{propertyId}/unit-types', {
    params: { path: { propertyId } },
    body: { ...body, idempotencyKey: crypto.randomUUID() },
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data as UnitType, error: null, blockers: [] };
};

export const updateUnitType = async (client: SessionClient, propertyId: string, unitTypeId: string, body: UnitTypeBody & { expectedVersion: number }): Promise<RequestResult<UnitType>> => {
  const response = await client.api.PATCH('/api/host/properties/{propertyId}/unit-types/{unitTypeId}', {
    params: { path: { propertyId, unitTypeId } },
    body: { ...body, idempotencyKey: crypto.randomUUID() },
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data as UnitType, error: null, blockers: [] };
};

export const archiveUnitType = async (client: SessionClient, propertyId: string, unitTypeId: string, expectedVersion: number): Promise<RequestResult<UnitType>> => {
  const response = await client.api.DELETE('/api/host/properties/{propertyId}/unit-types/{unitTypeId}', {
    params: { path: { propertyId, unitTypeId }, query: { expectedVersion } },
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data as UnitType, error: null, blockers: [] };
};

export const listMedia = async (client: SessionClient, propertyId: string): Promise<RequestResult<PropertyMedia[]>> => {
  const response = await client.api.GET('/api/host/properties/{propertyId}/media', { params: { path: { propertyId } } });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data as unknown as PropertyMedia[], error: null, blockers: [] };
};

export const addMedia = async (client: SessionClient, propertyId: string, input: { fileName: string; mimeType: string; contentBase64: string; altEn: string; altFr: string }): Promise<RequestResult<PropertyMedia>> => {
  const response = await client.api.POST('/api/host/properties/{propertyId}/media', {
    params: { path: { propertyId } },
    body: { ...input, clientKey: crypto.randomUUID() },
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data as unknown as PropertyMedia, error: null, blockers: [] };
};

export const updateMedia = async (client: SessionClient, propertyId: string, mediaId: string, body: { expectedVersion: number; altEn?: string; altFr?: string; isCover?: boolean; sortOrder?: number }): Promise<RequestResult<PropertyMedia>> => {
  const response = await client.api.PATCH('/api/host/properties/{propertyId}/media/{mediaId}', {
    params: { path: { propertyId, mediaId } },
    body,
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data as unknown as PropertyMedia, error: null, blockers: [] };
};

export const removeMedia = async (client: SessionClient, propertyId: string, mediaId: string, expectedVersion: number): Promise<RequestResult<{ mediaId: string; status: string }>> => {
  const response = await client.api.DELETE('/api/host/properties/{propertyId}/media/{mediaId}', {
    params: { path: { propertyId, mediaId } },
    body: { expectedVersion },
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null, blockers: [] };
};

export const listRevisions = async (client: SessionClient, propertyId: string, page: number): Promise<RequestResult<{ items: RevisionSummary[]; total: number }>> => {
  const response = await client.api.GET('/api/host/properties/{propertyId}/revisions', { params: { path: { propertyId }, query: { page, pageSize: 20 } } });
  if (response.error || !response.data) return fail(response.error);
  return { data: { items: response.data.data.items, total: response.data.data.total }, error: null, blockers: [] };
};

export const createRevision = async (client: SessionClient, propertyId: string): Promise<RequestResult<Revision>> => {
  const response = await client.api.POST('/api/host/properties/{propertyId}/revisions', { params: { path: { propertyId } }, body: { content: null } });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null, blockers: [] };
};

export const getRevision = async (client: SessionClient, propertyId: string, revisionId: string): Promise<RequestResult<Revision>> => {
  const response = await client.api.GET('/api/host/properties/{propertyId}/revisions/{revisionId}', { params: { path: { propertyId, revisionId } } });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null, blockers: [] };
};

export const updateRevision = async (client: SessionClient, propertyId: string, revisionId: string, expectedVersion: number, content: ListingContent): Promise<RequestResult<Revision>> => {
  const response = await client.api.PATCH('/api/host/properties/{propertyId}/revisions/{revisionId}', { params: { path: { propertyId, revisionId } }, body: { expectedVersion, content } });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null, blockers: [] };
};

export const submitRevision = async (client: SessionClient, propertyId: string, revisionId: string, expectedVersion: number): Promise<RequestResult<Revision>> => {
  const response = await client.api.POST('/api/host/properties/{propertyId}/revisions/{revisionId}/submit', { params: { path: { propertyId, revisionId } }, body: { expectedVersion } });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null, blockers: [] };
};

export const withdrawRevision = async (client: SessionClient, propertyId: string, revisionId: string, expectedVersion: number): Promise<RequestResult<Revision>> => {
  const response = await client.api.POST('/api/host/properties/{propertyId}/revisions/{revisionId}/withdraw', { params: { path: { propertyId, revisionId } }, body: { expectedVersion } });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null, blockers: [] };
};

export const listAmenities = async (client: SessionClient): Promise<RequestResult<Amenity[]>> => {
  const response = await client.api.GET('/api/public/amenities', { params: { query: { page: 1, pageSize: 100 } } });
  if (!response.data) return { data: null, error: 'INTERNAL_ERROR', blockers: [] };
  return { data: response.data.data.items.filter((item) => item.active), error: null, blockers: [] };
};
