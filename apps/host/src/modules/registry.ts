import { BarChart3, BadgePercent, Bell, Building2, CalendarDays, CalendarRange, ClipboardCheck, FolderOpen, LayoutDashboard, LayoutTemplate, ListChecks, MessageSquare, Settings, Star, Users, Wallet } from 'lucide-react';
import ReportsView from '@/views/reports/ReportsView';
import DashboardView from '@/views/dashboard/DashboardView';
import PropertiesView from '@/views/properties/PropertiesView';
import ReservationsView from '@/views/reservations/ReservationsView';
import MessagesView from '@/views/messages/MessagesView';
import CalendarView from '@/views/calendar/CalendarView';
import CreditsView from '@/views/credits/CreditsView';
import RatesView from '@/views/rates/RatesView';
import ReviewsView from '@/views/reviews/ReviewsView';
import TeamView from '@/views/team/TeamView';
import OnboardingView from '@/views/onboarding/OnboardingView';
import NotificationsView from '@/views/notifications/NotificationsView';
import SettingsView from '@/views/settings/SettingsView';
import TasksView from '@/views/tasks/TasksView';
import TemplatesView from '@/views/templates/TemplatesView';
import CasesView from '@/views/cases/CasesView';
import type { ModuleDefinition } from '@/types/modules/module';

export const moduleRegistry: ModuleDefinition[] = [
  {
    id: 'dashboard', component: DashboardView,
    labelKey: 'navDashboard',
    section: 'sectionWorkspace',
    route: '/',
    icon: LayoutDashboard,
    permission: 'analytics.read',
  },
  {
    id: 'reports', component: ReportsView, labelKey: 'navReports',
    section: 'sectionWorkspace', route: '/reports', icon: BarChart3, permission: 'analytics.read',
  },
  {
    id: 'properties', component: PropertiesView,
    labelKey: 'navProperties',
    section: 'sectionWorkspace',
    route: '/properties',
    icon: Building2,
    permission: 'properties.read',
  },
  {
    id: 'reservations', component: ReservationsView,
    labelKey: 'navReservations',
    section: 'sectionWorkspace',
    route: '/reservations',
    icon: CalendarDays,
    permission: 'reservations.read',
  },
  {
    id: 'tasks', component: TasksView,
    labelKey: 'navTasks',
    section: 'sectionWorkspace',
    route: '/tasks',
    icon: ListChecks,
    permissions: ['operations.checkin', 'operations.housekeeping', 'operations.maintenance'],
  },
  {
    id: 'cases', component: CasesView,
    labelKey: 'navCases',
    section: 'sectionWorkspace',
    route: '/cases',
    icon: FolderOpen,
    permission: 'reservations.read',
  },
  {
    id: 'templates', component: TemplatesView,
    labelKey: 'navTemplates',
    section: 'sectionWorkspace',
    route: '/accommodation-types',
    icon: LayoutTemplate,
    permission: 'properties.read',
  },
  {
    id: 'messages', component: MessagesView,
    labelKey: 'navMessages',
    section: 'sectionWorkspace',
    route: '/messages',
    icon: MessageSquare,
    permission: 'messages.write',
  },
  {
    id: 'calendar', component: CalendarView,
    labelKey: 'navCalendar',
    section: 'sectionWorkspace',
    route: '/calendar',
    icon: CalendarRange,
    permission: 'reservations.read',
  },
  {
    id: 'credits', component: CreditsView,
    labelKey: 'navCredits',
    section: 'sectionWorkspace',
    route: '/credits',
    icon: Wallet,
    permission: 'billing.read',
  },
  {
    id: 'rates', component: RatesView,
    labelKey: 'navRates',
    section: 'sectionWorkspace',
    route: '/rates',
    icon: BadgePercent,
    permission: 'properties.read',
  },
  {
    id: 'reviews', component: ReviewsView,
    labelKey: 'navReviews',
    section: 'sectionWorkspace',
    route: '/reviews',
    icon: Star,
    permission: 'reviews.write',
  },
  {
    id: 'team', component: TeamView,
    labelKey: 'navTeam',
    section: 'sectionWorkspace',
    route: '/team',
    icon: Users,
    permission: 'hosts.team',
  },
  {
    id: 'onboarding', component: OnboardingView,
    labelKey: 'navOnboarding',
    section: 'sectionAccount',
    route: '/onboarding',
    icon: ClipboardCheck,
  },
  {
    id: 'notifications', component: NotificationsView,
    labelKey: 'navNotifications',
    section: 'sectionAccount',
    route: '/notifications',
    icon: Bell,
  },
  {
    id: 'settings', component: SettingsView,
    labelKey: 'navSettings',
    section: 'sectionAccount',
    route: '/settings',
    icon: Settings,
  },
];
