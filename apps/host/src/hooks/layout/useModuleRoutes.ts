import { moduleRegistry } from '@/modules/registry';

export const useModuleRoutes = () => moduleRegistry.filter((module) => module.component);
