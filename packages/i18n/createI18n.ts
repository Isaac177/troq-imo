import { createInstance } from 'i18next';
import { initReactI18next } from 'react-i18next';
import { resources, type Language } from './resources';
import './typing';

export const defaultLanguage: Language = 'en';

export const createI18n = (language: Language = defaultLanguage) => {
  const instance = createInstance();
  instance.use(initReactI18next).init({
    resources,
    lng: language,
    fallbackLng: defaultLanguage,
    defaultNS: 'common',
    ns: Object.keys(resources[defaultLanguage]),
    interpolation: { escapeValue: false },
    initAsync: false,
  });
  return instance;
};
