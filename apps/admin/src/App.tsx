import { BrowserRouter, Navigate, Route, Routes } from 'react-router-dom';
import { SessionProvider } from '@mungini/session';
import UiCopyBridge from '@/components/app/UiCopyBridge';
import AppShell from '@/components/layout/AppShell';
import RequireSession from '@/components/layout/RequireSession';
import LoginView from '@/views/auth/LoginView';
import ModuleRoute from '@/components/layout/ModuleRoute';
import { moduleRegistry } from '@/modules/registry';
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
              {moduleRegistry.map((module) => <Route key={module.id} path={module.route} element={<ModuleRoute module={module} />} />)}
            </Route>
          </Route>
          <Route path="*" element={<Navigate to="/" replace />} />
        </Routes>
      </BrowserRouter>
      </SessionProvider>
    </UiCopyBridge>
  );
}
