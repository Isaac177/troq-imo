import { useSession } from '@mungini/session';
import { useTranslation } from '@mungini/i18n';
import type { ModuleDefinition } from '@/types/modules/module';

export const useModuleRoute = (module: ModuleDefinition) => {
  const { can } = useSession();
  const { t } = useTranslation('host');
  const allowed = module.permissions?.length
    ? module.permissions.some((permission) => can(permission))
    : !module.permission || can(module.permission);
  return { allowed, label: t('accessDenied'), Component: module.component };
};
