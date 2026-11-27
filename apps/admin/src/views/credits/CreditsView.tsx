import { Search } from 'lucide-react';
import { EmptyState, FadeIn, Input, PagedTable, Section, Select } from '@mungini/ui-web';
import CreditDetailDrawer from '@/components/credits/CreditDetailDrawer';
import CreditsSummary from '@/components/credits/CreditsSummary';
import { useAdminCredits } from '@/hooks/credits/useAdminCredits';

export default function CreditsView() {
  const view = useAdminCredits();

  if (view.forbidden) {
    return (
      <Section title={view.labels.title} description={view.labels.subtitle}>
        <EmptyState title={view.labels.forbidden} />
      </Section>
    );
  }

  return (
    <div className="flex flex-col gap-4 md:gap-6">
      <CreditsSummary cards={view.summaryCards} loading={view.summaryLoading} />
      <Section
        title={view.labels.title}
        description={view.labels.subtitle}
        actions={
          <div className="flex w-full flex-col gap-2 sm:w-auto sm:flex-row">
            <div className="w-full sm:w-[180px]">
              <Select options={view.statusOptions} value={view.status} onValueChange={view.setStatus} ariaLabel={view.labels.status} />
            </div>
            <div className="w-full sm:w-[260px]">
              <Input type="search" value={view.search} onChange={view.changeSearch} placeholder={view.labels.search} aria-label={view.labels.search} leadingIcon={<Search />} />
            </div>
          </div>
        }
      >
        <FadeIn swapKey={view.tableKey}>
          <PagedTable
            rows={view.rows}
            columns={view.columns}
            getRowId={view.rowId}
            page={view.page}
            pageSize={view.pageSize}
            total={view.total}
            onPageChange={view.setPage}
            ariaLabel={view.labels.title}
            loading={view.loading}
            error={view.error ?? undefined}
            onRowClick={view.openRow}
            emptyState={<EmptyState title={view.labels.empty} />}
          />
        </FadeIn>
      </Section>
      <CreditDetailDrawer hostId={view.selected} hostName={view.selectedName} onClose={view.closeDrawer} onChanged={view.onChanged} />
    </div>
  );
}
