import { Outlet, Navigate } from 'react-router-dom';
import { useSession } from '@mungini/session';
import { useTranslation } from '@mungini/i18n';

export default function RequireSession() {
  const { status } = useSession();
  const { i18n } = useTranslation();
  const label = i18n.getFixedT(i18n.language, 'common')('loading') as string;

  if (status === 'loading') {
    return (
      <main className="flex min-h-dvh items-center justify-center bg-muted text-body text-muted-foreground">
        <span className="animate-fade-in">…</span>
      </main>
    );
  }

  if (status === 'anonymous') return <Navigate to="/login" replace />;

  return <Outlet />;
}
