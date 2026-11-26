import { Search } from 'lucide-react';
import { Button, Dialog, EmptyState, Input, PagedTable, Section, Select, StatusDot, type TableColumn } from '@mungini/ui-web';
import { useApplications } from '@/hooks/applications/useApplications';
import type { AdminApplicationView } from '@/services/admin/admin.api';

const tones: Record<string, 'neutral' | 'info' | 'success' | 'danger' | 'muted'> = {
  DRAFT: 'neutral',
  PENDING_REVIEW: 'info',
  APPROVED: 'success',
  REJECTED: 'danger',
  WITHDRAWN: 'muted',
};

export default function ApplicationsView() {
  const view = useApplications();

  const columns: TableColumn<AdminApplicationView>[] = [
    {
      key: 'applicant',
      header: view.labels.application,
      cell: (row) => (
        <div className="min-w-0">
          <p className="truncate font-medium text-foreground">{row.displayName}</p>
          <p className="truncate text-label text-muted-foreground">{row.applicantName || row.applicantEmail || row.applicantUserId}</p>
        </div>
      ),
    },
    {
      key: 'status',
      header: view.labels.status,
      cell: (row) => <StatusDot tone={tones[row.status] ?? 'muted'} label={view.labels.statusLabel(row.status)} />,
      align: 'right',
    },
    {
      key: 'submitted',
      header: view.labels.submitted,
      cell: (row) => (row.submittedAt ? row.submittedAt.slice(0, 10) : ''),
      align: 'right',
      hideBelow: 'sm',
    },
  ];

  return (
    <>
      <Section
        title={view.labels.title}
        description={view.labels.subtitle}
        actions={
          <div className="flex w-full flex-col gap-2 sm:w-auto sm:flex-row">
            <div className="w-full sm:w-[150px]">
              <Select
                options={[
                  { value: 'PENDING_REVIEW', label: view.labels.statusLabel('PENDING_REVIEW') },
                  { value: 'APPROVED', label: view.labels.statusLabel('APPROVED') },
                  { value: 'REJECTED', label: view.labels.statusLabel('REJECTED') },
                  { value: 'DRAFT', label: view.labels.statusLabel('DRAFT') },
                ]}
                value={view.status}
                onValueChange={view.setStatus}
                placeholder={view.labels.all}
                ariaLabel={view.labels.status}
              />
            </div>
            <div className="w-full sm:w-[220px]">
              <Input
                type="search"
                value={view.search}
                onChange={(event) => view.setSearch(event.target.value)}
                placeholder={view.labels.search}
                aria-label={view.labels.search}
                leadingIcon={<Search />}
              />
            </div>
          </div>
        }
      >
        {view.error ? <p className="text-label text-destructive">{view.error}</p> : null}
        <PagedTable
          rows={view.items}
          columns={columns}
          getRowId={(row) => row.id}
          page={view.page}
          pageSize={view.pageSize}
          total={view.total}
          onPageChange={view.setPage}
          ariaLabel={view.labels.title}
          loading={view.loading}
          onRowClick={(row) => void view.open(row)}
          emptyState={<EmptyState title={view.labels.empty} />}
        />
      </Section>

      <Dialog
        open={view.selected !== null}
        onOpenChange={(open) => (open ? undefined : view.close())}
        title={view.selected?.displayName ?? ''}
        description={view.selected ? view.labels.statusLabel(view.selected.status) : ''}
        size="lg"
        footer={
          view.selected?.status === 'PENDING_REVIEW' ? (
            <div className="flex flex-wrap items-end justify-end gap-2">
              <div className="w-[150px]">
                <Select
                  options={[
                    { value: 'APPROVE', label: view.labels.approve },
                    { value: 'REJECT', label: view.labels.reject },
                  ]}
                  value={view.decision}
                  onValueChange={view.setDecision}
                  placeholder={view.labels.decide}
                  ariaLabel={view.labels.decide}
                />
              </div>
              <div className="w-[190px]">
                <Select
                  options={view.reasons.map((value) => ({ value, label: view.labels.reasonLabel(value) }))}
                  value={view.reason}
                  onValueChange={view.setReason}
                  placeholder={view.labels.chooseReason}
                  ariaLabel={view.labels.reason}
                  disabled={!view.decision}
                />
              </div>
              <Button variant="primary" onClick={() => void view.decide()} disabled={!view.decision || !view.reason || view.busy}>
                {view.labels.decide}
              </Button>
            </div>
          ) : null
        }
      >
        <div className="flex flex-col gap-4">
          <div>
            <p className="text-label text-muted-foreground">{view.labels.documents}</p>
            {view.selected && view.selected.documents.length > 0 ? (
              <ul className="mt-1 divide-y divide-border">
                {view.selected.documents.map((document) => (
                  <li key={document.id} className="flex flex-wrap items-center justify-between gap-2 py-2">
                    <div className="min-w-0">
                      <p className="truncate text-body font-medium text-foreground">{document.fileName}</p>
                      <p className="text-label text-muted-foreground">
                        {view.labels.documentKindLabel(document.kind)} · {view.labels.documentStatus(document.status)}
                      </p>
                    </div>
                    {document.status === 'PENDING' && view.selected?.status === 'PENDING_REVIEW' ? (
                      <div className="flex items-center gap-1">
                        <Button variant="outline" size="sm" onClick={() => void view.reviewDocument(document.id, 'VERIFIED')}>
                          {view.labels.verify}
                        </Button>
                        <div className="w-[150px]">
                          <Select
                            options={view.documentReasons.map((value) => ({ value, label: view.labels.reasonLabel(value) }))}
                            value={view.docReason}
                            onValueChange={view.setDocReason}
                            placeholder={view.labels.rejectDocument}
                            ariaLabel={view.labels.rejectDocument}
                          />
                        </div>
                        <Button variant="ghost" size="sm" disabled={!view.docReason} onClick={() => void view.reviewDocument(document.id, 'REJECTED')}>
                          {view.labels.reject}
                        </Button>
                      </div>
                    ) : null}
                  </li>
                ))}
              </ul>
            ) : (
              <EmptyState title={view.labels.noDocuments} />
            )}
          </div>
        </div>
      </Dialog>
    </>
  );
}
