import { Search } from 'lucide-react';
import { EmptyState, Input, PagedTable, Section, Select } from '@mungini/ui-web';
import ReviewModerationDialog from '@/components/reviews/ReviewModerationDialog';
import { useAdminReviews } from '@/hooks/reviews/useAdminReviews';

export default function ReviewsView() {
  const view = useAdminReviews();
  return (
    <>
      <Section
        title={view.labels.title}
        description={view.labels.subtitle}
        actions={
          <div className="flex w-full flex-col gap-2 sm:w-auto sm:flex-row">
            <div className="w-full sm:w-[180px]">
              <Select options={view.statusOptions} value={view.status} onValueChange={view.setStatus} ariaLabel={view.labels.status} />
            </div>
            <div className="w-full sm:w-[150px]">
              <Select options={view.ratingOptions} value={view.rating} onValueChange={view.setRating} ariaLabel={view.labels.rating} />
            </div>
            <div className="w-full sm:w-[240px]">
              <Input type="search" value={view.search} onChange={(event) => view.setSearch(event.target.value)} placeholder={view.labels.search} aria-label={view.labels.search} leadingIcon={<Search />} />
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
          onRowClick={(row) => view.open(row)}
          emptyState={<EmptyState title={view.labels.empty} />}
        />
      </Section>
      <ReviewModerationDialog view={view} />
    </>
  );
}
