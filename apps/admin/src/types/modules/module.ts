import type { ComponentType } from 'react';
import type { LucideIcon } from 'lucide-react';

export type Permission = `${string}.${string}`;

export type ModuleDefinition = {
  id: string;
  labelKey: string;
  section: string;
  route: string;
  icon: LucideIcon;
  permission?: Permission;
};

export type AdminRoute = {
  path: string;
  component: ComponentType;
};
