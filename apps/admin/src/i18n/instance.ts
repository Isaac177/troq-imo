import { createI18n } from '@mungini/i18n';

const stored = typeof localStorage === 'undefined' ? null : localStorage.getItem('mungini-language');

export const i18n = createI18n(stored === 'fr' ? 'fr' : 'en');
