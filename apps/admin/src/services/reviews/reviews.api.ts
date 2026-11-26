import type { SessionClient } from '@mungini/session';
import { apiErrorCode } from '@mungini/session';
import type { AdminReview, RequestResult, Review } from '@/types/reviews/reviews';

export type PagedResult<T> = { items: T[]; total: number; page: number; pageSize: number };

const fail = <T>(error: unknown): RequestResult<T> => ({ data: null, error: apiErrorCode(error) });

export type ReviewQuery = { status: string; rating: string; search: string; page: number; pageSize: number };

export const listReviews = async (client: SessionClient, query: ReviewQuery): Promise<RequestResult<PagedResult<AdminReview>>> => {
  const response = await client.api.GET('/api/admin/reviews', {
    params: {
      query: {
        page: query.page,
        pageSize: query.pageSize,
        ...(query.status ? { status: query.status } : {}),
        ...(query.rating ? { rating: Number(query.rating) } : {}),
        ...(query.search.trim() ? { search: query.search.trim() } : {}),
      },
    },
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const decideReview = async (client: SessionClient, reviewId: string, status: string, reasonCode: string | null): Promise<RequestResult<Review>> => {
  const response = await client.api.POST('/api/admin/reviews/{reviewId}/moderation', {
    params: { path: { reviewId } },
    body: { status, ...(reasonCode ? { reasonCode } : {}) },
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const listCities = (client: SessionClient) => client.api.GET('/api/public/cities');
