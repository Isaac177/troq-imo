import { BarChart3, Bell, Building2, ClipboardList, FileCheck2, FolderOpen, LayoutDashboard, ListChecks, ShieldAlert, Star, Users, Wallet, CalendarDays } from 'lucide-react';
import ReportsView from '@/views/reports/ReportsView';
import DashboardView from '@/views/dashboard/DashboardView';
import ApplicationsView from '@/views/applications/ApplicationsView';
import HostsView from '@/views/hosts/HostsView';
import GuestsView from '@/views/guests/GuestsView';
import ReservationsView from '@/views/reservations/ReservationsView';
import CalendarView from '@/views/calendar/CalendarView';
import ReviewsView from '@/views/reviews/ReviewsView';
import CasesView from '@/views/cases/CasesView';
import ListingReviewsView from '@/views/listings/ListingReviewsView';
import ClaimsView from '@/views/claims/ClaimsView';
import PropertiesView from '@/views/properties/PropertiesView';
import NotificationsView from '@/views/notifications/NotificationsView';
import TasksView from '@/views/tasks/TasksView';
import CreditsView from '@/views/credits/CreditsView';
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
    id: 'reports', component: ReportsView, labelKey: 'navReports',
    section: 'sectionPlatform', route: '/reports', icon: BarChart3,
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
    id: 'tasks', component: TasksView, labelKey: 'navTasks',
    section: 'sectionOperations', route: '/tasks', icon: ListChecks,
    permission: 'properties.moderate',
  },
  {
    id: 'credits', component: CreditsView, labelKey: 'navCredits',
    section: 'sectionOperations', route: '/credits', icon: Wallet,
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
  {
    id: 'listings', component: ListingReviewsView, labelKey: 'navListings',
    section: 'sectionOperations', route: '/listings', icon: FileCheck2,
    permission: 'properties.moderate',
  },
  {
    id: 'claims', component: ClaimsView, labelKey: 'navClaims',
    section: 'sectionOperations', route: '/claims', icon: ShieldAlert,
    permission: 'properties.moderate',
  },
  {
    id: 'properties', component: PropertiesView, labelKey: 'navProperties',
    section: 'sectionOperations', route: '/properties', icon: Building2,
    permission: 'properties.moderate',
  },
  {
    id: 'notifications', component: NotificationsView, labelKey: 'navNotifications',
    section: 'sectionOperations', route: '/notifications', icon: Bell,
  },
];
