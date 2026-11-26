import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { useRouter } from 'next/navigation';

export const useSiteHeader = () => {
  const { i18n } = useTranslation();
  const router = useRouter();
  const { session, status, logout } = useSession();
  const t = i18n.getFixedT(i18n.language, 'guest') as unknown as (key: string) => string;
  const common = i18n.getFixedT(i18n.language, 'common') as unknown as (key: string) => string;
  const name = [session?.user.firstName, session?.user.lastName].filter(Boolean).join(' ') || session?.user.email || '';

  return {
    status,
    name,
    labels: {
      appName: common('appName'),
      home: t('homeTitle'),
      account: t('account'),
      signIn: t('signIn'),
      register: t('register'),
      signOut: t('signOut'),
      light: common('switchToLight'),
      dark: common('switchToDark'),
      language: common('language'),
    },
    language: i18n.language,
    changeLanguage: (language: string) => {
      void i18n.changeLanguage(language);
      document.documentElement.lang = language;
      document.cookie = `mungini-language=${language}; path=/; max-age=31536000; samesite=lax`;
      router.refresh();
    },
    logout: () => {
      void logout();
    },
  };
};
