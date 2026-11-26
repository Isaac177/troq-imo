import { BrowserRouter, Navigate, Route, Routes } from 'react-router-dom';
import { SessionProvider } from '@mungini/session';
import UiCopyBridge from '@/components/app/UiCopyBridge';
import AppShell from '@/components/layout/AppShell';
import RequireSession from '@/components/layout/RequireSession';
import LoginView from '@/views/auth/LoginView';
import DashboardView from '@/views/dashboard/DashboardView';
import PropertiesView from '@/views/properties/PropertiesView';
import ReservationsView from '@/views/reservations/ReservationsView';
import CalendarView from '@/views/calendar/CalendarView';
import CreditsView from '@/views/credits/CreditsView';
import RatesView from '@/views/rates/RatesView';
import ReviewsView from '@/views/reviews/ReviewsView';
import SettingsView from '@/views/settings/SettingsView';
import NotificationsView from '@/views/notifications/NotificationsView';
import TeamView from '@/views/team/TeamView';
import OnboardingView from '@/views/onboarding/OnboardingView';
import { sessionClient } from '@/services/session/sessionClient';

export default function App() {
  return (
    <UiCopyBridge>
      <SessionProvider client={sessionClient}>
      <BrowserRouter>
        <Routes>
          <Route path="/login" element={<LoginView />} />
          <Route element={<RequireSession />}>
            <Route element={<AppShell />}>
              <Route index element={<DashboardView />} />
              <Route path="properties" element={<PropertiesView />} />
              <Route path="reservations" element={<ReservationsView />} />
              <Route path="calendar" element={<CalendarView />} />
              <Route path="credits" element={<CreditsView />} />
              <Route path="rates" element={<RatesView />} />
              <Route path="reviews" element={<ReviewsView />} />
              <Route path="settings" element={<SettingsView />} />
              <Route path="notifications" element={<NotificationsView />} />
              <Route path="team" element={<TeamView />} />
              <Route path="onboarding" element={<OnboardingView />} />
            </Route>
          </Route>
          <Route path="*" element={<Navigate to="/" replace />} />
        </Routes>
      </BrowserRouter>
      </SessionProvider>
    </UiCopyBridge>
  );
}
