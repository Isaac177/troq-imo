import { EmptyState, PagedTable, Section, Select } from '@mungini/ui-web';
import ListingReviewDrawer from '@/components/listings/ListingReviewDrawer';
import { useListingQueue } from '@/hooks/listings/useListingQueue';

export default function ListingReviewsView() {
  const view = useListingQueue();
  return (
    <>
      <Section
        title={view.labels.title}
        description={view.labels.subtitle}
        actions={<div className="w-full sm:w-[200px]"><Select options={view.statusOptions} value={view.status} onValueChange={view.setStatus} ariaLabel={view.labels.status} /></div>}
      >
        {view.error ? <p role="alert" className="text-label text-destructive">{view.error}</p> : null}
        {view.notice ? <p role="status" className="text-label text-muted-foreground">{view.notice}</p> : null}
        <PagedTable
          rows={view.rows}
          columns={view.columns}
          getRowId={(row) => row.revisionId}
          page={view.page}
          pageSize={view.pageSize}
          total={view.total}
          onPageChange={view.setPage}
          ariaLabel={view.labels.title}
          loading={view.loading}
          onRowClick={(row) => view.setSelected(row.revisionId)}
          emptyState={<EmptyState title={view.labels.empty} />}
        />
      </Section>
      <ListingReviewDrawer revisionId={view.selected} onClose={() => view.setSelected(null)} onChanged={view.onChanged} />
    </>
  );
}
