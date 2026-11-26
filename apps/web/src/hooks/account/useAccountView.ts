import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';

export const useAccountView = () => {
  const { i18n } = useTranslation();
  const { session, status, logout } = useSession();
  const t = i18n.getFixedT(i18n.language, 'guest') as unknown as (key: string) => string;

  return {
    status,
    session,
    labels: {
      title: t('accountTitle'),
      email: t('email'),
      phone: t('phone'),
      city: t('city'),
      signOut: t('signOut'),
      signIn: t('signIn'),
      noSession: t('signInSubtitle'),
    },
    signOut: () => {
      void logout();
    },
  };
};
