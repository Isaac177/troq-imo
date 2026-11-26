import { Search } from 'lucide-react';
import { EmptyState, Input, PagedTable, Section, Select, Switch } from '@mungini/ui-web';
import CaseDetailDrawer from '@/components/cases/CaseDetailDrawer';
import { useAdminCases } from '@/hooks/cases/useAdminCases';

export default function CasesView() {
  const view = useAdminCases();
  return (
    <>
      <Section
        title={view.labels.title}
        description={view.labels.subtitle}
        actions={
          <div className="flex w-full flex-col gap-2 sm:w-auto sm:flex-row">
            <div className="w-full sm:w-[170px]">
              <Select options={view.stateOptions} value={view.state} onValueChange={view.setState} ariaLabel={view.labels.state} />
            </div>
            <div className="w-full sm:w-[190px]">
              <Select options={view.typeOptions} value={view.caseType} onValueChange={view.setCaseType} ariaLabel={view.labels.caseType} />
            </div>
            <div className="w-full sm:w-[260px]">
              <Input type="search" value={view.search} onChange={(event) => view.setSearch(event.target.value)} placeholder={view.labels.search} aria-label={view.labels.search} leadingIcon={<Search />} />
            </div>
            <div className="flex h-9 items-center">
              <Switch checked={view.overdue} onCheckedChange={view.setOverdue} label={view.labels.overdueOnly} />
            </div>
          </div>
        }
      >
        {view.error ? <p role="alert" className="text-label text-destructive">{view.error}</p> : null}
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
          onRowClick={(row) => view.setSelected(row.id)}
          emptyState={<EmptyState title={view.labels.empty} />}
        />
      </Section>
      <CaseDetailDrawer caseId={view.selected} onClose={() => view.setSelected(null)} onChanged={view.onChanged} />
    </>
  );
}
