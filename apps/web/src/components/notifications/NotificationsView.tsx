'use client';

import Link from 'next/link';
import { BellOff, CheckCheck } from 'lucide-react';
import { Button, EmptyState, FadeIn, Pagination, Skeleton, UnderlineTabs } from '@mungini/ui-web';
import { NotificationRow } from '@/components/notifications/NotificationRow';
import { useNotifications } from '@/hooks/notifications/useNotifications';

export const NotificationsView = () => {
  const view = useNotifications();

  return (
    <main className="mx-auto flex w-full max-w-3xl flex-col gap-4 px-3 py-8 md:px-6">
      <div className="flex flex-wrap items-start justify-between gap-2">
        <div className="flex flex-col gap-1">
          <h1 className="text-title font-semibold">{view.labels.title}</h1>
          {view.status === 'authenticated' && view.unreadSummary ? <p className="text-label text-muted-foreground">{view.unreadSummary}</p> : null}
        </div>
        {view.status === 'authenticated' ? (
          <Button
            variant="outline"
            size="sm"
            icon={<CheckCheck />}
            collapseLabel
            aria-label={view.labels.markAllRead}
            title={view.labels.markAllRead}
            disabled={view.unread === null || view.unread === 0 || view.busy}
            loading={view.busy}
            onClick={view.markAllRead}
          >
            {view.labels.markAllRead}
          </Button>
        ) : null}
      </div>

      {view.status === 'loading' ? <p className="text-body text-muted-foreground">{view.labels.loading}</p> : null}

      {view.status === 'anonymous' ? (
        <p className="text-body">
          {view.labels.signInPrompt}{' '}
          <Link className="text-primary hover:underline" href="/login">{view.labels.signIn}</Link>
        </p>
      ) : null}

      {view.status === 'authenticated' ? (
        <>
          <UnderlineTabs items={view.tabs} value={view.filter} onChange={view.changeFilter} ariaLabel={view.labels.filter} />
          {view.actionError ? <p role="alert" className="text-label text-destructive">{view.actionError}</p> : null}
          {view.error ? (
            <div role="alert" className="flex items-center gap-3 rounded-card border border-border bg-card p-4 text-body">
              <span>{view.labels.error}</span>
              <Button size="sm" variant="outline" onClick={view.retry}>{view.labels.retry}</Button>
            </div>
          ) : (
            <section aria-busy={view.loading || undefined} className={view.loading && view.data ? 'overflow-hidden rounded-card border border-border bg-card opacity-60 transition-opacity duration-200 ease-in-out' : 'overflow-hidden rounded-card border border-border bg-card transition-opacity duration-200 ease-in-out'}>
              {view.data ? (
                <FadeIn swapKey={`${view.filter}-${view.page}`}>
                  {view.data.total === 0 ? (
                    <EmptyState
                      icon={<BellOff />}
                      title={view.filter === 'unread' ? view.labels.emptyUnread : view.labels.emptyAll}
                      description={view.filter === 'unread' ? view.labels.emptyUnreadHint : view.labels.emptyAllHint}
                      action={<Button asChild variant="outline" size="sm"><Link href="/">{view.labels.explore}</Link></Button>}
                    />
                  ) : null}
                  {view.items.length > 0 ? (
                    <ul className="flex flex-col">
                      {view.items.map((item) => <NotificationRow key={item.id} row={view.describe(item)} />)}
                    </ul>
                  ) : null}
                  {view.data.total > view.data.pageSize ? (
                    <Pagination page={view.page} pageSize={view.data.pageSize} total={view.data.total} onPageChange={view.changePage} />
                  ) : null}
                </FadeIn>
              ) : (
                <div className="flex flex-col gap-3 p-4">
                  <Skeleton shape="block" />
                  <Skeleton shape="block" />
                  <Skeleton shape="block" />
                </div>
              )}
            </section>
          )}
        </>
      ) : null}
    </main>
  );
};
