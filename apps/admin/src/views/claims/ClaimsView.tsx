import { Search } from 'lucide-react';
import { EmptyState, Input, PagedTable, Section, Select, StatCard } from '@mungini/ui-web';
import ClaimDetailDrawer from '@/components/claims/ClaimDetailDrawer';
import { useAdminClaims } from '@/hooks/claims/useAdminClaims';

export default function ClaimsView() {
  const view = useAdminClaims();
  return (
    <>
      <div className="flex flex-col gap-4 md:gap-6">
        {view.summary ? (
          <div className="grid grid-cols-2 gap-3 md:grid-cols-3">
            <StatCard label={view.labels.total} value={String(view.summary.total)} />
            <StatCard label={view.labels.undecided} value={String(view.summary.undecided)} />
            <StatCard label={view.labels.claimed} value={view.money(view.summary.claimed)} />
            <StatCard label={view.labels.approved} value={view.money(view.summary.approved)} />
            <StatCard label={view.labels.paid} value={view.money(view.summary.paid)} />
            <StatCard label={view.labels.payoutPending} value={view.money(view.summary.payoutPending)} />
          </div>
        ) : null}

        <Section
          title={view.labels.title}
          description={view.labels.subtitle}
          actions={
            <div className="flex w-full flex-col gap-2 sm:w-auto sm:flex-row">
              <div className="w-full sm:w-[180px]">
                <Select options={view.statusOptions} value={view.status} onValueChange={view.setStatus} ariaLabel={view.labels.status} />
              </div>
              <div className="w-full sm:w-[260px]">
                <Input type="search" value={view.search} onChange={(event) => view.setSearch(event.target.value)} placeholder={view.labels.search} aria-label={view.labels.search} leadingIcon={<Search />} />
              </div>
            </div>
          }
        >
          {view.error ? <p role="alert" className="text-label text-destructive">{view.error}</p> : null}
          <PagedTable
            rows={view.rows}
            columns={view.columns}
            getRowId={(row) => row.caseId}
            page={view.page}
            pageSize={view.pageSize}
            total={view.total}
            onPageChange={view.setPage}
            ariaLabel={view.labels.title}
            loading={view.loading}
            onRowClick={(row) => view.setSelected(row.caseId)}
            emptyState={<EmptyState title={view.labels.empty} />}
          />
        </Section>
      </div>
      <ClaimDetailDrawer caseId={view.selected} onClose={() => view.setSelected(null)} onChanged={view.onChanged} />
    </>
  );
}
