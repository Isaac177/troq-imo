import type { ComponentType } from 'react';
import type { LucideIcon } from 'lucide-react';

export type Permission = `${string}.${string}`;

export type SidebarSection = string;

export type ModuleRoute = {
  path: string;
  component: ComponentType;
  permission?: Permission;
};

export type ModuleDefinition = {
  id: string;
  component?: ComponentType;
  labelKey: string;
  section: SidebarSection;
  route: string;
  icon: LucideIcon;
  permission?: Permission;
  permissions?: Permission[];
};
