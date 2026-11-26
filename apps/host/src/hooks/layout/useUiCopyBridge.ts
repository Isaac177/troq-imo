import { useMemo } from 'react';
import { useTranslation } from '@mungini/i18n';
import { createUiCopy } from '@mungini/ui-web';

export const useUiCopyBridge = () => {
  const { i18n } = useTranslation();
  return useMemo(
    () => createUiCopy(i18n.getFixedT(i18n.language, 'ui') as unknown as (key: string) => string, i18n.language),
    [i18n, i18n.language],
  );
};
