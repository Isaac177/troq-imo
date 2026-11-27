import type { components } from '@mungini/api';

export type RatePlan = components['schemas']['PlanView'];
export type RateRule = components['schemas']['RuleView'];
export type Promotion = components['schemas']['PromotionView'];
export type Policy = components['schemas']['PolicyView'];
export type UnitType = components['schemas']['Envelope_Page_UnitTypeView']['data']['items'][number];
export type PropertyOption = { value: string; label: string };
export type RequestResult<T> = { data: T | null; error: string | null };
export type PricePreview = components['schemas']['PricePreviewView'];
export type PricePreviewInput = components['schemas']['QuoteRequest'];
