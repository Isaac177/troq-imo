import type { components } from '@mungini/api';

export type Template = Omit<components['schemas']['TemplateView'], 'accessibility'> & { accessibility: string[] };

export type TemplateInput = components['schemas']['TemplateInput'];

export type TemplateMedia = components['schemas']['PropertyMediaView'] & { url: string | null };

export type TemplateCategory = components['schemas']['CategoryView'];

export type TemplatePage = { items: Template[]; total: number; page: number; pageSize: number };

export type TemplateListQuery = { hostId: string; search: string; status: string; categoryCode: string; page: number; pageSize: number };

export type TemplateMediaInput = { fileName: string; mimeType: string; contentBase64: string; altEn: string; altFr: string };

export type TemplateMediaUpdate = { expectedVersion: number; altEn?: string; altFr?: string; isCover?: boolean; sortOrder?: number };

export type RequestResult<T> = { data: T | null; error: string | null };
