import { useTranslation } from '@mungini/i18n';

export const useHomeScreen = () => {
  const { t } = useTranslation();
  return { appName: t('appName') };
};
