import { Link } from 'react-router-dom';
import { Plus } from 'lucide-react';
import { Button, EmptyState, PagedTable, Section, Select } from '@mungini/ui-web';
import CaseDetailDrawer from '@/components/cases/CaseDetailDrawer';
import CaseOpenDialog from '@/components/cases/CaseOpenDialog';
import { useHostCases } from '@/hooks/cases/useHostCases';
import { useOpenCase } from '@/hooks/cases/useOpenCase';

export default function CasesView() {
  const view = useHostCases();
  const opener = useOpenCase({ hostId: view.hostId, canWrite: view.canWrite, onOpened: view.onCaseOpened });

  if (!view.hasHost) {
    return (
      <Section title={view.labels.title} description={view.labels.subtitle}>
        <EmptyState
          title={view.labels.noHost}
          action={<Button asChild variant="primary" size="sm"><Link to="/onboarding">{view.labels.noHostAction}</Link></Button>}
        />
      </Section>
    );
  }

  return (
    <>
      <Section
        title={view.labels.title}
        description={view.labels.subtitle}
        actions={
          <div className="flex w-full flex-col gap-2 sm:w-auto sm:flex-row sm:items-center">
            {view.hosts.length > 1 ? (
              <div className="w-full sm:w-[170px]">
                <Select options={view.hosts} value={view.hostId} onValueChange={view.setHostId} ariaLabel={view.labels.host} />
              </div>
            ) : null}
            <div className="w-full sm:w-[170px]">
              <Select options={view.stateOptions} value={view.stateValue} onValueChange={view.setState} ariaLabel={view.labels.state} />
            </div>
            {view.canWrite ? (
              <Button
                variant="primary"
                size="sm"
                icon={<Plus />}
                collapseLabel
                aria-label={view.labels.openCase}
                title={view.labels.openCase}
                onClick={opener.openDialog}
              >
                {view.labels.openCase}
              </Button>
            ) : null}
          </div>
        }
      >
        {view.notice ? <p role="status" className="text-label text-muted-foreground">{view.notice}</p> : null}
        {view.forbidden ? (
          <EmptyState title={view.labels.forbidden} />
        ) : view.error ? (
          <div role="alert"><EmptyState title={view.error} /></div>
        ) : (
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
        )}
      </Section>
      <CaseDetailDrawer caseId={view.selected} canWrite={view.canWrite} onClose={view.closeDetail} onChanged={view.refresh} />
      <CaseOpenDialog view={opener} />
    </>
  );
}
