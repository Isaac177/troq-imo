import type { components } from '@mungini/api';

export type Preference = components['schemas']['PreferenceView'];
export type PreferenceList = components['schemas']['PreferenceList'];
export type RequestResult<T> = { data: T | null; error: string | null };
