import { Navigate } from 'react-router-dom';
import { Button, Input } from '@mungini/ui-web';
import { useLoginView } from '@/hooks/auth/useLoginView';

export default function LoginView() {
  const view = useLoginView();

  if (view.authenticated) return <Navigate to="/" replace />;

  return (
    <main className="flex min-h-dvh items-center justify-center bg-muted px-4 py-8">
      <section className="w-full max-w-[420px] rounded-card border border-border bg-card p-5 md:p-6">
        <h1 className="text-title font-semibold">{view.labels.title}</h1>
        <p className="mt-1 text-label text-muted-foreground">{view.labels.subtitle}</p>
        {view.loading ? (
          <p className="mt-6 text-body text-muted-foreground">{view.labels.loading}</p>
        ) : (
          <form className="mt-5 flex flex-col gap-3" onSubmit={view.submit}>
            <label className="flex flex-col gap-1 text-label text-muted-foreground">
              {view.labels.identifier}
              <Input
                value={view.identifier}
                onChange={(event) => view.setIdentifier(event.target.value)}
                placeholder={view.labels.identifierPlaceholder}
                autoComplete="username"
                required
              />
            </label>
            <label className="flex flex-col gap-1 text-label text-muted-foreground">
              {view.labels.password}
              <Input
                type="password"
                value={view.password}
                onChange={(event) => view.setPassword(event.target.value)}
                placeholder={view.labels.passwordPlaceholder}
                autoComplete="current-password"
                required
              />
            </label>
            {view.error ? <p className="text-label text-destructive">{view.error}</p> : null}
            <Button type="submit" variant="primary" disabled={view.busy} className="mt-1">
              {view.busy ? view.labels.signingIn : view.labels.submit}
            </Button>
            <p className="text-micro text-muted-foreground">{view.labels.demoHint}</p>
          </form>
        )}
      </section>
    </main>
  );
}
