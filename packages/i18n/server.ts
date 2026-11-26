import { createInstance } from 'i18next';
import { resources, type Language } from './resources';

export const createServerI18n = (language: Language) => {
  const instance = createInstance();
  instance.init({
    resources,
    lng: language,
    fallbackLng: 'en',
    defaultNS: 'common',
    ns: Object.keys(resources.en),
    interpolation: { escapeValue: false },
    initAsync: false,
  });
  return instance;
};
