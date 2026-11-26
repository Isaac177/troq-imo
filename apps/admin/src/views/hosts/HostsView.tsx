import { Search } from 'lucide-react';
import { Button, Dialog, EmptyState, Input, PagedTable, Section, Select, StatusDot, type TableColumn } from '@mungini/ui-web';
import { useHosts } from '@/hooks/hosts/useHosts';
import type { AdminHostView } from '@/services/admin/admin.api';

export default function HostsView() {
  const view = useHosts();

  const columns: TableColumn<AdminHostView>[] = [
    {
      key: 'host',
      header: view.labels.host,
      cell: (row) => (
        <div className="min-w-0">
          <p className="truncate font-medium text-foreground">{row.displayName}</p>
          <p className="truncate text-label text-muted-foreground">{row.cityId}</p>
        </div>
      ),
    },
    {
      key: 'status',
      header: view.labels.status,
      cell: (row) => <StatusDot tone={row.status === 'ACTIVE' ? 'success' : 'danger'} label={view.labels.statusLabel(row.status)} />,
    },
    {
      key: 'members',
      header: view.labels.members,
      align: 'right',
      cell: (row) => <span className="tabular-nums">{row.memberCount}</span>,
      hideBelow: 'sm',
    },
    {
      key: 'properties',
      header: view.labels.properties,
      align: 'right',
      cell: (row) => <span className="tabular-nums">{row.propertyCount}</span>,
      hideBelow: 'md',
    },
    {
      key: 'actions',
      header: '',
      align: 'right',
      cell: (row) => (
        <Button variant="ghost" size="sm" onClick={() => view.open(row)}>
          {view.labels.status}
        </Button>
      ),
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
                  { value: 'ACTIVE', label: view.labels.statusLabel('ACTIVE') },
                  { value: 'SUSPENDED', label: view.labels.statusLabel('SUSPENDED') },
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
          onRowClick={(row) => view.open(row)}
          emptyState={<EmptyState title={view.labels.empty} />}
        />
      </Section>

      <Dialog
        open={view.selected !== null}
        onOpenChange={(open) => (open ? undefined : view.close())}
        title={view.selected?.displayName ?? ''}
        description={view.selected ? view.labels.statusLabel(view.selected.status) : ''}
        footer={
          <div className="flex justify-end gap-2">
            <Button variant="ghost" onClick={view.close}>
              {view.labels.cancel}
            </Button>
            <Button variant="primary" onClick={() => void view.apply()} disabled={!view.action || !view.reason || view.busy}>
              {view.labels.confirm}
            </Button>
          </div>
        }
      >
        <div className="flex flex-col gap-3 sm:flex-row">
          <div className="flex-1">
            <Select
              options={[
                { value: 'SUSPEND', label: view.labels.suspend },
                { value: 'REINSTATE', label: view.labels.reinstate },
              ]}
              value={view.action}
              onValueChange={view.setAction}
              placeholder={view.labels.status}
              ariaLabel={view.labels.status}
            />
          </div>
          <div className="flex-1">
            <Select
              options={view.reasons.map((value) => ({ value, label: view.labels.reasonLabel(value) }))}
              value={view.reason}
              onValueChange={view.setReason}
              placeholder={view.labels.chooseReason}
              ariaLabel={view.labels.reason}
              disabled={!view.action}
            />
          </div>
        </div>
      </Dialog>
    </>
  );
}
