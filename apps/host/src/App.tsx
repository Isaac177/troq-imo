import { BrowserRouter, Navigate, Route, Routes } from 'react-router-dom';
import { SessionProvider } from '@mungini/session';
import UiCopyBridge from '@/components/app/UiCopyBridge';
import AppShell from '@/components/layout/AppShell';
import ModuleRoute from '@/components/layout/ModuleRoute';
import { useModuleRoutes } from '@/hooks/layout/useModuleRoutes';
import RequireSession from '@/components/layout/RequireSession';
import LoginView from '@/views/auth/LoginView';
import { sessionClient } from '@/services/session/sessionClient';

export default function App() {
  const moduleRoutes = useModuleRoutes();
  return (
    <UiCopyBridge>
      <SessionProvider client={sessionClient}>
      <BrowserRouter>
        <Routes>
          <Route path="/login" element={<LoginView />} />
          <Route element={<RequireSession />}>
            <Route element={<AppShell />}>
              {moduleRoutes.map((module) => <Route key={module.id} path={module.route} element={<ModuleRoute module={module} />} />)}
            </Route>
          </Route>
          <Route path="*" element={<Navigate to="/" replace />} />
        </Routes>
      </BrowserRouter>
      </SessionProvider>
    </UiCopyBridge>
  );
}
