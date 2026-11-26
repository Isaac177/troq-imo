'use client';

import Link from 'next/link';
import { Navigate } from '@/components/navigation/Navigate';
import { Button, Input, Select } from '@mungini/ui-web';
import { useRegisterView } from '@/hooks/auth/useRegisterView';

export default function RegisterView() {
  const view = useRegisterView();

  if (view.authenticated) return <Navigate to="/account" />;

  return (
    <main className="mx-auto flex w-full max-w-[460px] flex-col gap-4 px-3 py-8 md:px-6">
      <div>
        <h1 className="text-title font-semibold">{view.labels.title}</h1>
        <p className="mt-1 text-label text-muted-foreground">{view.labels.subtitle}</p>
        <p className="mt-2 text-label text-muted-foreground">
          {view.step === 1 ? view.labels.stepContact : view.step === 2 ? view.labels.stepCode : view.labels.stepDetails}
        </p>
      </div>

      {view.step === 1 ? (
        <form onSubmit={view.sendCode} className="flex flex-col gap-3 rounded-card border border-border bg-card p-4">
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.email}
            <Input type="email" value={view.email} onChange={(event) => view.setEmail(event.target.value)} autoComplete="email" required />
          </label>
          {view.error ? <p className="text-label text-destructive">{view.error}</p> : null}
          <Button type="submit" variant="primary" disabled={view.sending}>
            {view.labels.sendCode}
          </Button>
        </form>
      ) : null}

      {view.step === 2 ? (
        <form onSubmit={view.verify} className="flex flex-col gap-3 rounded-card border border-border bg-card p-4">
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.code}
            <Input inputMode="numeric" value={view.code} onChange={(event) => view.setCode(event.target.value)} autoComplete="one-time-code" required />
          </label>
          {view.error ? <p className="text-label text-destructive">{view.error}</p> : null}
          <Button type="submit" variant="primary" disabled={view.sending}>
            {view.labels.verify}
          </Button>
        </form>
      ) : null}

      {view.step === 3 ? (
        <form onSubmit={view.finish} className="flex flex-col gap-3 rounded-card border border-border bg-card p-4">
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.firstName}
            <Input value={view.firstName} onChange={(event) => view.setFirstName(event.target.value)} autoComplete="given-name" />
          </label>
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.lastName}
            <Input value={view.lastName} onChange={(event) => view.setLastName(event.target.value)} autoComplete="family-name" />
          </label>
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.city}
            <Select
              options={view.cities.map((item) => ({ value: item.code, label: item.name }))}
              value={view.city}
              onValueChange={view.setCity}
              ariaLabel={view.labels.city}
            />
          </label>
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.password}
            <Input type="password" value={view.password} onChange={(event) => view.setPassword(event.target.value)} autoComplete="new-password" required />
          </label>
          {view.error ? <p className="text-label text-destructive">{view.error}</p> : null}
          <Button type="submit" variant="primary" disabled={view.sending}>
            {view.labels.create}
          </Button>
        </form>
      ) : null}

      <p className="text-label text-muted-foreground">
        {view.labels.haveAccount}{' '}
        <Link href="/login" className="text-primary underline-offset-4 hover:underline">
          {view.labels.signIn}
        </Link>
      </p>
    </main>
  );
}
