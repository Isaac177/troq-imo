import { BellOff, CheckCheck } from 'lucide-react';
import { Badge, Button, EmptyState, PagedTable, Section, SegmentedControl } from '@mungini/ui-web';
import { useAdminNotifications } from '@/hooks/notifications/useAdminNotifications';

export default function NotificationsView() {
  const view = useAdminNotifications();
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
          disabled={view.unreadCount === 0}
          onClick={() => void view.markAllRead()}
          aria-label={view.labels.markAllRead}
          title={view.labels.markAllRead}
        >
          {view.labels.markAllRead}
        </Button>
      }
    >
      <div className="flex flex-wrap items-center justify-between gap-2">
        <SegmentedControl items={view.filterItems} value={view.filter} onChange={view.changeFilter} ariaLabel={view.labels.filter} size="sm" />
        <span className="flex items-center gap-2">
          <span className="text-label text-muted-foreground">{view.labels.unread}</span>
          <Badge variant={view.unreadCount > 0 ? 'primary' : 'secondary'}>{view.unreadCount}</Badge>
        </span>
      </div>
      {view.actionError ? <p role="alert" className="text-label text-destructive">{view.actionError}</p> : null}
      {view.notice ? <p role="status" className="text-label text-muted-foreground">{view.notice}</p> : null}
      <PagedTable
        rows={view.rows}
        columns={view.columns}
        getRowId={(row) => row.id}
        page={view.page}
        pageSize={view.pageSize}
        total={view.total}
        onPageChange={view.changePage}
        ariaLabel={view.labels.title}
        loading={view.loading}
        error={view.error}
        onRowClick={view.open}
        emptyState={<EmptyState title={view.emptyTitle} description={view.emptyHint} icon={<BellOff />} />}
      />
    </Section>
  );
}
