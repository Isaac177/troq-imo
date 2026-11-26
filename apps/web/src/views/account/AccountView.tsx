'use client';

import Link from 'next/link';
import { Button } from '@mungini/ui-web';
import { useTranslation } from '@mungini/i18n';
import { Invitations } from '@/components/account/Invitations';
import { ProfileForm } from '@/components/account/ProfileForm';
import { useAccountView } from '@/hooks/account/useAccountView';

export default function AccountView() {
  const view = useAccountView();
  const { i18n } = useTranslation();
  const loading = i18n.getFixedT(i18n.language, 'common')('loading') as string;

  if (view.status === 'loading') {
    return <main className="mx-auto w-full max-w-3xl px-3 py-8 text-body text-muted-foreground md:px-6">{loading}</main>;
  }

  if (view.status !== 'authenticated' || !view.session) {
    return (
      <main className="mx-auto flex w-full max-w-[420px] flex-col gap-3 px-3 py-8 md:px-6">
        <p className="text-body text-muted-foreground">{view.labels.noSession}</p>
        <Link href="/login" className="text-primary underline-offset-4 hover:underline">
          {view.labels.signIn}
        </Link>
      </main>
    );
  }

  return (
    <main className="mx-auto flex w-full max-w-3xl flex-col gap-4 px-3 py-8 md:px-6">
      <div className="flex items-center justify-between gap-2">
        <h1 className="text-title font-semibold">{view.labels.title}</h1>
        <Button variant="outline" size="sm" onClick={view.signOut}>
          {view.labels.signOut}
        </Button>
      </div>
      <ProfileForm />
      <dl className="grid grid-cols-1 gap-3 rounded-card border border-border bg-card p-4 sm:grid-cols-2">
        <div>
          <dt className="text-label text-muted-foreground">{view.labels.email}</dt>
          <dd className="text-body">{view.session.user.email || '—'}</dd>
        </div>
        <div>
          <dt className="text-label text-muted-foreground">{view.labels.phone}</dt>
          <dd className="text-body">{view.session.user.phone || '—'}</dd>
        </div>
        <div>
          <dt className="text-label text-muted-foreground">{view.labels.city}</dt>
          <dd className="text-body">{view.session.user.city}</dd>
        </div>
      </dl>
      <Link href="/favorites" className="text-body text-primary underline-offset-4 hover:underline">{view.labels.favorites}</Link>
      <Link href="/trips" className="text-body text-primary underline-offset-4 hover:underline">{view.labels.trips}</Link>
      <Invitations />
    </main>
  );
}
