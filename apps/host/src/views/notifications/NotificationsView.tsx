import { BellOff, CheckCheck } from 'lucide-react';
import { Badge, Button, cn, EmptyState, FadeIn, Pagination, Section, SegmentedControl } from '@mungini/ui-web';
import NotificationListSkeleton from '@/components/notifications/NotificationListSkeleton';
import NotificationRow from '@/components/notifications/NotificationRow';
import { useHostNotifications } from '@/hooks/notifications/useHostNotifications';

export default function NotificationsView() {
  const view = useHostNotifications();

  return (
    <Section
      title={view.labels.title}
      description={view.labels.subtitle}
      actions={
        <Button
          variant="outline"
          size="sm"
          icon={<CheckCheck />}
          collapseLabel
          loading={view.markingAll}
          onClick={() => void view.markAllRead()}
          aria-label={view.labels.markAllRead}
          title={view.labels.markAllRead}
        >
          {view.labels.markAllRead}
        </Button>
      }
    >
      <div className="flex flex-wrap items-center justify-between gap-2">
        <SegmentedControl items={view.filterItems} value={view.filter} onChange={view.changeFilter} ariaLabel={view.labels.filter} />
        <span className="flex items-center gap-2">
          <span className="text-label text-muted-foreground">{view.labels.unread}</span>
          <Badge variant={view.unreadCount > 0 ? 'primary' : 'secondary'}>{view.unreadCount}</Badge>
        </span>
      </div>
      {view.error ? <p role="alert" className="text-label text-destructive">{view.error}</p> : null}
      {view.notice ? <p role="status" className="text-label text-muted-foreground">{view.notice}</p> : null}
      {view.loading && view.rowCount === 0 ? <NotificationListSkeleton /> : null}
      {!view.loading && !view.error && view.rowCount === 0 ? (
        <EmptyState title={view.emptyTitle} description={view.emptyHint} icon={<BellOff />} />
      ) : null}
      {view.rowCount > 0 ? (
        <FadeIn swapKey={`${view.filter}-${view.page}-${view.refreshKey}`}>
          <div aria-busy={view.loading} className={cn('divide-y divide-border transition-opacity duration-200 ease-in-out', view.loading && 'opacity-60')}>
            {view.rows.map((row) => <NotificationRow key={row.id} row={row} />)}
          </div>
        </FadeIn>
      ) : null}
      <Pagination page={view.page} pageSize={view.pageSize} total={view.total} onPageChange={view.changePage} />
    </Section>
  );
}
