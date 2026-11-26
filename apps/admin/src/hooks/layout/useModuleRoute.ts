import { useSession } from '@mungini/session';
import { useTranslation } from '@mungini/i18n';
import type { ModuleDefinition } from '@/types/modules/module';

export const useModuleRoute = (module: ModuleDefinition) => {
  const { can } = useSession();
  const { t } = useTranslation('admin');
  return { allowed: !module.permission || can(module.permission), label: t('accessDenied'), Component: module.component };
};
