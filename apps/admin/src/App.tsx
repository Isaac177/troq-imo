import { BrowserRouter, Navigate, Route, Routes } from 'react-router-dom';
import { SessionProvider } from '@mungini/session';
import UiCopyBridge from '@/components/app/UiCopyBridge';
import AppShell from '@/components/layout/AppShell';
import RequireSession from '@/components/layout/RequireSession';
import LoginView from '@/views/auth/LoginView';
import DashboardView from '@/views/dashboard/DashboardView';
import ApplicationsView from '@/views/applications/ApplicationsView';
import HostsView from '@/views/hosts/HostsView';
import GuestsView from '@/views/guests/GuestsView';
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
              <Route path="applications" element={<ApplicationsView />} />
              <Route path="hosts" element={<HostsView />} />
              <Route path="guests" element={<GuestsView />} />
            </Route>
          </Route>
          <Route path="*" element={<Navigate to="/" replace />} />
        </Routes>
      </BrowserRouter>
      </SessionProvider>
    </UiCopyBridge>
  );
}
