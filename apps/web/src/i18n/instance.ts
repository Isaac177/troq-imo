import { createI18n } from '@mungini/i18n';

const stored = typeof document === 'undefined' ? null : document.cookie.match(/(?:^|; )mungini-language=(en|fr)/)?.[1];

export const i18n = createI18n(stored === 'fr' ? 'fr' : 'en');
