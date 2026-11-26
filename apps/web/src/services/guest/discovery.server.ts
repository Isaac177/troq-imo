import type { components } from '@mungini/api';
import { apiBaseUrl } from '@/env';

export type SearchResponse = components['schemas']['SearchResponse'];
export type PropertyDetailView = components['schemas']['PropertyDetailView'];

const base = apiBaseUrl.replace(/\/$/, '');

const load = async <T>(path: string, city: string): Promise<T | null> => {
  try {
    const response = await fetch(`${base}${path}`, {
      headers: { 'x-city': city },
      next: { revalidate: 60 },
    });
    if (!response.ok) return null;
    const body = (await response.json()) as { data?: T };
    return body.data ?? null;
  } catch {
    return null;
  }
};

export const listStays = (city: string, search: string, page: number) => {
  const query = new URLSearchParams({ page: String(page), pageSize: '12', locale: 'en' });
  if (search.trim()) query.set('search', search.trim());
  return load<SearchResponse>(`/api/public/properties?${query.toString()}`, city);
};

export const getStay = (city: string, propertyId: string) =>
  load<PropertyDetailView>(`/api/public/properties/${encodeURIComponent(propertyId)}?locale=en`, city);

export const servedCities = async (): Promise<{ code: string; name: string }[]> => {
  const data = await load<{ code: string; name: string }[]>('/api/public/cities', 'GOMA');
  return data ?? [
    { code: 'GOMA', name: 'Goma' },
    { code: 'KIN', name: 'Kinshasa' },
  ];
};
