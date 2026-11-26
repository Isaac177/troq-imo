import { useEffect, useState } from 'react';
import { useLocation } from 'react-router-dom';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { moduleRegistry } from '@/modules/registry';

export const useAppShell = () => {
  const { i18n } = useTranslation();
  const { session, city, setCity, logout, client, can } = useSession();
  const location = useLocation();
  const [menuOpen, setMenuOpen] = useState(false);
  const [cities, setCities] = useState<{ value: string; label: string }[]>([]);
  const admin = i18n.getFixedT(i18n.language, 'admin') as unknown as (key: string) => string;
  const common = i18n.getFixedT(i18n.language, 'common') as unknown as (key: string) => string;

  useEffect(() => {
    let active = true;
    void client.api.GET('/api/public/cities').then(({ data }) => {
      if (!active || !data) return;
      setCities(data.data.map((item) => ({ value: item.code, label: item.name })));
    });
    return () => {
      active = false;
    };
  }, [client]);

  useEffect(() => {
    setMenuOpen(false);
  }, [location.pathname]);

  const modules = moduleRegistry.filter((module) => !module.permission || can(module.permission));
  const name = [session?.user.firstName, session?.user.lastName].filter(Boolean).join(' ') || session?.user.email || '';

  return {
    modules,
    activePath: location.pathname,
    menuOpen,
    toggleMenu: () => setMenuOpen((value) => !value),
    closeMenu: () => setMenuOpen(false),
    city,
    cities,
    setCity: (value: string) => {
      void setCity(value);
    },
    logout: () => {
      void logout();
    },
    name,
    admin,
    common,
    language: i18n.language,
    changeLanguage: (language: string) => {
      void i18n.changeLanguage(language);
      document.documentElement.lang = language;
      try {
        localStorage.setItem('mungini-language', language);
      } catch {
        return;
      }
    },
  };
};
