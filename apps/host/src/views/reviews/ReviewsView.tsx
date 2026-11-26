import { Link } from 'react-router-dom';
import { Button, EmptyState, PagedTable, Section, Select } from '@mungini/ui-web';
import ReviewReplyDialog from '@/components/reviews/ReviewReplyDialog';
import { useHostReviews } from '@/hooks/reviews/useHostReviews';

export default function ReviewsView() {
  const view = useHostReviews();

  if (!view.hasHost) {
    return (
      <Section title={view.labels.title} description={view.labels.subtitle}>
        <EmptyState title={view.labels.noHost} action={<Button asChild variant="primary" size="sm"><Link to="/onboarding">{view.labels.noHostAction}</Link></Button>} />
      </Section>
    );
  }

  return (
    <>
      <Section
        title={view.labels.title}
        description={view.labels.subtitle}
        actions={
          <div className="flex w-full flex-col gap-2 sm:w-auto sm:flex-row">
            {view.hosts.length > 1 ? (
              <div className="w-full sm:w-[160px]">
                <Select options={view.hosts.map((host) => ({ value: host.hostId, label: host.displayName }))} value={view.hostId} onValueChange={view.setHostId} ariaLabel={view.labels.title} />
              </div>
            ) : null}
            <div className="w-full sm:w-[200px]">
              <Select options={view.properties} value={view.propertyId} onValueChange={view.setPropertyId} placeholder={view.labels.chooseProperty} ariaLabel={view.labels.property} />
            </div>
            <div className="w-full sm:w-[180px]">
              <Select options={view.statusOptions} value={view.status} onValueChange={view.setStatus} ariaLabel={view.labels.status} />
            </div>
          </div>
        }
      >
        {view.error ? <p role="alert" className="text-label text-destructive">{view.error}</p> : null}
        {view.notice ? <p role="status" className="text-label text-muted-foreground">{view.notice}</p> : null}
        <PagedTable
          rows={view.rows}
          columns={view.columns}
          getRowId={(row) => row.id}
          page={view.page}
          pageSize={view.pageSize}
          total={view.total}
          onPageChange={view.setPage}
          ariaLabel={view.labels.title}
          loading={view.loading}
          emptyState={<EmptyState title={view.labels.empty} />}
        />
      </Section>
      <ReviewReplyDialog view={view} />
    </>
  );
}
