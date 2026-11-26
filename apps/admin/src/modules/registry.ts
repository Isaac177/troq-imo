import { Building2, ClipboardList, FolderOpen, LayoutDashboard, Star, Users, CalendarDays } from 'lucide-react';
import DashboardView from '@/views/dashboard/DashboardView';
import ApplicationsView from '@/views/applications/ApplicationsView';
import HostsView from '@/views/hosts/HostsView';
import GuestsView from '@/views/guests/GuestsView';
import ReservationsView from '@/views/reservations/ReservationsView';
import CalendarView from '@/views/calendar/CalendarView';
import ReviewsView from '@/views/reviews/ReviewsView';
import CasesView from '@/views/cases/CasesView';
import type { ModuleDefinition } from '@/types/modules/module';

export const moduleRegistry: ModuleDefinition[] = [
  {
    id: 'dashboard',
    component: DashboardView,
    labelKey: 'navDashboard',
    section: 'sectionPlatform',
    route: '/',
    icon: LayoutDashboard,
    permission: 'properties.moderate',
  },
  {
    id: 'applications',
    component: ApplicationsView,
    labelKey: 'navApplications',
    section: 'sectionOperations',
    route: '/applications',
    icon: ClipboardList,
    permission: 'hosts.review_applications',
  },
  {
    id: 'hosts',
    component: HostsView,
    labelKey: 'navHosts',
    section: 'sectionOperations',
    route: '/hosts',
    icon: Building2,
    permission: 'hosts.review_applications',
  },
  {
    id: 'guests',
    component: GuestsView,
    labelKey: 'navGuests',
    section: 'sectionOperations',
    route: '/guests',
    icon: Users,
    permission: 'properties.moderate',
  },
  {
    id: 'calendar', component: CalendarView, labelKey: 'navCalendar',
    section: 'sectionOperations', route: '/calendar', icon: CalendarDays,
    permission: 'properties.moderate',
  },
  {
    id: 'reservations', component: ReservationsView, labelKey: 'navReservations',
    section: 'sectionOperations', route: '/reservations', icon: CalendarDays,
    permission: 'properties.moderate',
  },
  {
    id: 'reviews', component: ReviewsView, labelKey: 'navReviews',
    section: 'sectionOperations', route: '/reviews', icon: Star,
    permission: 'reviews.moderate',
  },
  {
    id: 'cases', component: CasesView, labelKey: 'navCases',
    section: 'sectionOperations', route: '/cases', icon: FolderOpen,
    permission: 'properties.moderate',
  },
];
