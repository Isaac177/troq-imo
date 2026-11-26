import type { SessionClient } from '@mungini/session';
import { apiErrorCode } from '@mungini/session';
import type { RequestResult, Review } from '@/types/reviews/reviews';

export type PagedResult<T> = { items: T[]; total: number; page: number; pageSize: number };

const fail = <T>(error: unknown): RequestResult<T> => ({ data: null, error: apiErrorCode(error) });

export const listReviews = async (client: SessionClient, propertyId: string, status: string, page: number, pageSize: number): Promise<RequestResult<PagedResult<Review>>> => {
  const response = await client.api.GET('/api/host/properties/{propertyId}/reviews', {
    params: { path: { propertyId }, query: { page, pageSize, ...(status ? { status } : {}) } },
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const replyToReview = async (client: SessionClient, reviewId: string, reply: string): Promise<RequestResult<Review>> => {
  const response = await client.api.POST('/api/host/reviews/{reviewId}/reply', {
    params: { path: { reviewId } },
    body: { reply },
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const listCities = (client: SessionClient) => client.api.GET('/api/public/cities');
