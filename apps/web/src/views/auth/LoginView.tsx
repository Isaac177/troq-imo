'use client';

import Link from 'next/link';
import { Navigate } from '@/components/navigation/Navigate';
import { Button, Input } from '@mungini/ui-web';
import { useLoginView } from '@/hooks/auth/useLoginView';

export default function LoginView() {
  const view = useLoginView();

  if (view.authenticated) return <Navigate to="/account" />;

  return (
    <main className="mx-auto flex w-full max-w-[420px] flex-col gap-4 px-3 py-8 md:px-6">
      <div>
        <h1 className="text-title font-semibold">{view.labels.title}</h1>
        <p className="mt-1 text-label text-muted-foreground">{view.labels.subtitle}</p>
      </div>
      <form onSubmit={view.submit} className="flex flex-col gap-3 rounded-card border border-border bg-card p-4">
        <label className="flex flex-col gap-1 text-label text-muted-foreground">
          {view.labels.email}
          <Input type="email" value={view.email} onChange={(event) => view.setEmail(event.target.value)} autoComplete="username" required />
        </label>
        <label className="flex flex-col gap-1 text-label text-muted-foreground">
          {view.labels.password}
          <Input type="password" value={view.password} onChange={(event) => view.setPassword(event.target.value)} autoComplete="current-password" required />
        </label>
        {view.error ? <p className="text-label text-destructive">{view.error}</p> : null}
        <Button type="submit" variant="primary" disabled={view.busy}>
          {view.labels.submit}
        </Button>
      </form>
      <p className="text-label text-muted-foreground">
        {view.labels.noAccount}{' '}
        <Link href="/register" className="text-primary underline-offset-4 hover:underline">
          {view.labels.register}
        </Link>
      </p>
    </main>
  );
}
