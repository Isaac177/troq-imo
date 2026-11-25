import * as en from './locales/en';
import * as fr from './locales/fr';

export const resources = {
  en: { ...en },
  fr: { ...fr } satisfies typeof en,
};

export type Resources = typeof resources;
export type Language = keyof Resources;
