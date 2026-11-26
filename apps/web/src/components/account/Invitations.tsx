'use client';

import { Button } from '@mungini/ui-web';
import { useInvitations } from '@/hooks/account/useInvitations';

export const Invitations = () => {
  const view = useInvitations();
  return (
    <section className="rounded-card border border-border bg-card p-4 transition-all duration-200 ease-in-out" aria-labelledby="invitations-title">
      <h2 id="invitations-title" className="text-title font-semibold">{view.labels.title}</h2>
      {view.error && <div role="alert" className="mt-3 flex items-center gap-3 text-body"><span>{view.labels.error}</span><Button size="sm" variant="outline" onClick={view.retry}>{view.labels.retry}</Button></div>}
      {view.accepted && <p role="status" className="mt-3 text-body">{view.labels.accepted}</p>}
      {view.loading ? <p className="mt-3 text-body text-muted-foreground">{view.labels.loading}</p> : null}
      {!view.loading && !view.error && !view.data?.items.length ? <p className="mt-3 text-body text-muted-foreground">{view.labels.empty}</p> : null}
      {!view.loading && view.data?.items.map((invitation) => (
        <div key={invitation.id} className="flex flex-col gap-3 border-t border-border py-4 first:mt-3 sm:flex-row sm:items-center sm:justify-between">
          <div className="min-w-0 space-y-1">
            <h3 className="break-words text-body font-semibold">{invitation.hostName}</h3>
            <p className="text-label text-muted-foreground">{view.labels.expires}: {view.formatDate(invitation.expiresAt)}</p>
            <p className="text-label text-muted-foreground">{view.labels.permissions}: {invitation.permissions.map(view.permission).join(', ')}</p>
          </div>
          <Button size="sm" loading={view.accepting === invitation.id} disabled={view.accepting !== null} onClick={() => void view.accept(invitation.id)}>{view.labels.accept}</Button>
        </div>
      ))}
      {view.data && view.data.total > view.data.pageSize && <div className="flex items-center justify-between gap-2 border-t border-border pt-3">
        <Button size="sm" variant="outline" disabled={view.page <= 1 || view.loading} onClick={view.previous}>{view.labels.previous}</Button>
        <span className="text-label">{view.labels.page} {view.page}</span>
        <Button size="sm" variant="outline" disabled={view.page * view.data.pageSize >= view.data.total || view.loading} onClick={view.next}>{view.labels.next}</Button>
      </div>}
    </section>
  );
};
