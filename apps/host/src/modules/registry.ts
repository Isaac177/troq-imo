import { Building2, ClipboardCheck, LayoutDashboard, Users } from 'lucide-react';
import type { ModuleDefinition } from '@/types/modules/module';

export const moduleRegistry: ModuleDefinition[] = [
  {
    id: 'dashboard',
    labelKey: 'navDashboard',
    section: 'sectionWorkspace',
    route: '/',
    icon: LayoutDashboard,
    permission: 'analytics.read',
  },
  {
    id: 'properties',
    labelKey: 'navProperties',
    section: 'sectionWorkspace',
    route: '/properties',
    icon: Building2,
    permission: 'properties.read',
  },
  {
    id: 'team',
    labelKey: 'navTeam',
    section: 'sectionWorkspace',
    route: '/team',
    icon: Users,
    permission: 'hosts.team',
  },
  {
    id: 'onboarding',
    labelKey: 'navOnboarding',
    section: 'sectionAccount',
    route: '/onboarding',
    icon: ClipboardCheck,
  },
];
