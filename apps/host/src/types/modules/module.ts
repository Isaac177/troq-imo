import type { ComponentType } from 'react';

export type Permission = `${string}.${string}`;

export type SidebarSection = string;

export type ModuleRoute = {
  path: string;
  component: ComponentType;
  permission?: Permission;
};

export type ModuleDefinition = {
  id: string;
  labelKey: string;
  section: SidebarSection;
  routes: ModuleRoute[];
  permission: Permission;
};
