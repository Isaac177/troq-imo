import { Building2, ClipboardList, LayoutDashboard, Users } from 'lucide-react';
import type { ModuleDefinition } from '@/types/modules/module';

export const moduleRegistry: ModuleDefinition[] = [
  {
    id: 'dashboard',
    labelKey: 'navDashboard',
    section: 'sectionPlatform',
    route: '/',
    icon: LayoutDashboard,
    permission: 'properties.moderate',
  },
  {
    id: 'applications',
    labelKey: 'navApplications',
    section: 'sectionOperations',
    route: '/applications',
    icon: ClipboardList,
    permission: 'hosts.review_applications',
  },
  {
    id: 'hosts',
    labelKey: 'navHosts',
    section: 'sectionOperations',
    route: '/hosts',
    icon: Building2,
    permission: 'hosts.review_applications',
  },
  {
    id: 'guests',
    labelKey: 'navGuests',
    section: 'sectionOperations',
    route: '/guests',
    icon: Users,
    permission: 'properties.moderate',
  },
];
