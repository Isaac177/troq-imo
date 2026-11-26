import { useTranslation } from '@mungini/i18n';

export const useNotificationsLink = () => {
  const { i18n } = useTranslation();
  const fixed = i18n as unknown as { getFixedT: (language: string, namespace: string) => (key: string) => string };
  const t = fixed.getFixedT(i18n.language, 'guestNotifications');
  return { open: t('open') };
};
