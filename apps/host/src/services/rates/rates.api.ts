import type { SessionClient } from '@mungini/session';
import { apiErrorCode } from '@mungini/session';
import type { Policy, PricePreview, PricePreviewInput, Promotion, RatePlan, RateRule, RequestResult, UnitType } from '@/types/rates/rates';

const fail = <T>(error: unknown): RequestResult<T> => ({ data: null, error: apiErrorCode(error) });

export type PagedResult<T> = { items: T[]; total: number; page: number; pageSize: number };

export const previewPrice = async (client: SessionClient, body: PricePreviewInput): Promise<RequestResult<PricePreview>> => {
  const response = await client.api.POST('/api/host/price-preview', { body });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const listProperties = (client: SessionClient, hostId: string, page: number, pageSize: number) =>
  client.api.GET('/api/host/properties', { params: { query: { hostId, page, pageSize } } });

export const listUnitTypes = async (client: SessionClient, propertyId: string): Promise<RequestResult<PagedResult<UnitType>>> => {
  const response = await client.api.GET('/api/host/properties/{propertyId}/unit-types', { params: { path: { propertyId }, query: { page: 1, pageSize: 100 } } });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const listPolicies = async (client: SessionClient, kind: string): Promise<RequestResult<Policy[]>> => {
  const response = await client.api.GET('/api/host/rate-policies', { params: { query: { kind } } });
  if (!response.data) return { data: null, error: 'INTERNAL_ERROR' };
  return { data: response.data.data, error: null };
};

export const listPlans = async (client: SessionClient, propertyId: string, page: number, pageSize: number): Promise<RequestResult<PagedResult<RatePlan>>> => {
  const response = await client.api.GET('/api/host/properties/{propertyId}/rate-plans', { params: { path: { propertyId }, query: { page, pageSize } } });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export type CreatePlanBody = { unitTypeId: string; occupancyBasis: string; mealPlan: string; bookingMode: string; cancellationPolicyVersionId: string; paymentPolicyVersionId: string; childNightlyAmount: string };

export const createPlan = async (client: SessionClient, propertyId: string, body: CreatePlanBody): Promise<RequestResult<RatePlan>> => {
  const response = await client.api.POST('/api/host/properties/{propertyId}/rate-plans', { params: { path: { propertyId } }, body: { ...body, idempotencyKey: crypto.randomUUID() } });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export type UpdatePlanBody = { expectedVersion: number; status?: string; bookingMode?: string; mealPlan?: string; childNightlyAmount?: string };

export const updatePlan = async (client: SessionClient, propertyId: string, planId: string, body: UpdatePlanBody): Promise<RequestResult<RatePlan>> => {
  const response = await client.api.PATCH('/api/host/properties/{propertyId}/rate-plans/{ratePlanId}', { params: { path: { propertyId, ratePlanId: planId } }, body: { ...body, idempotencyKey: crypto.randomUUID() } });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const listRules = async (client: SessionClient, propertyId: string, planId: string, page: number, pageSize: number): Promise<RequestResult<PagedResult<RateRule>>> => {
  const response = await client.api.GET('/api/host/properties/{propertyId}/rate-plans/{ratePlanId}/rules', { params: { path: { propertyId, ratePlanId: planId }, query: { page, pageSize } } });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export type CreateRuleBody = { startDate: string | null; endDate: string | null; weekdays: number[]; adjustmentType: string; amount: string; priority: number };

export const createRule = async (client: SessionClient, propertyId: string, planId: string, body: CreateRuleBody): Promise<RequestResult<RateRule>> => {
  const response = await client.api.POST('/api/host/properties/{propertyId}/rate-plans/{ratePlanId}/rules', { params: { path: { propertyId, ratePlanId: planId } }, body: { ...body, idempotencyKey: crypto.randomUUID() } });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const removeRule = async (client: SessionClient, propertyId: string, planId: string, ruleId: string): Promise<RequestResult<{ id: string }>> => {
  const response = await client.api.DELETE('/api/host/properties/{propertyId}/rate-plans/{ratePlanId}/rules/{ruleId}', { params: { path: { propertyId, ratePlanId: planId, ruleId }, query: { idempotencyKey: crypto.randomUUID() } } });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const listPromotions = async (client: SessionClient, propertyId: string, page: number, pageSize: number): Promise<RequestResult<PagedResult<Promotion>>> => {
  const response = await client.api.GET('/api/host/properties/{propertyId}/promotions', { params: { path: { propertyId }, query: { page, pageSize } } });
  if (!response.data) return { data: null, error: 'INTERNAL_ERROR' };
  return { data: response.data.data, error: null };
};

export type CreatePromotionBody = { ratePlanId: string | null; minNights: number; discountPercent: string; capAmount: string | null; startDate: string | null; endDate: string | null; priority: number; redemptionLimit: number | null };

export const createPromotion = async (client: SessionClient, propertyId: string, body: CreatePromotionBody): Promise<RequestResult<Promotion>> => {
  const response = await client.api.POST('/api/host/properties/{propertyId}/promotions', { params: { path: { propertyId } }, body: { ...body, idempotencyKey: crypto.randomUUID() } });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const updatePromotion = async (client: SessionClient, propertyId: string, promotionId: string, status: string): Promise<RequestResult<Promotion>> => {
  const response = await client.api.PATCH('/api/host/properties/{propertyId}/promotions/{promotionId}', { params: { path: { propertyId, promotionId } }, body: { status, idempotencyKey: crypto.randomUUID() } });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};
