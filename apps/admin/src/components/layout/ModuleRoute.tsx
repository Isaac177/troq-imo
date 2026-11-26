import { useModuleRoute } from '@/hooks/layout/useModuleRoute';
import type { ModuleDefinition } from '@/types/modules/module';

export default function ModuleRoute({ module }: { module: ModuleDefinition }) {
  const view = useModuleRoute(module);
  return view.allowed ? <view.Component /> : <p className="text-body text-muted-foreground">{view.label}</p>;
}
