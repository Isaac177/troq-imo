import { useTranslation } from '@mungini/i18n';

export const useHomeView = () => {
  const { t } = useTranslation();
  return { appName: t('appName'), lightLabel: t('switchToLight'), darkLabel: t('switchToDark') };
};
