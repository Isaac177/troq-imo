import type { components } from '@mungini/api';

type ApiError = components['schemas']['ErrorBody'];

export const apiErrorCode = (error: unknown): string => {
  const value = error as ApiError | undefined;
  return value?.error?.code ?? 'INTERNAL_ERROR';
};
