import { Search } from 'lucide-react';
import { Dialog, EmptyState, Input, PagedTable, Section, Select, type TableColumn } from '@mungini/ui-web';
import { useGuests } from '@/hooks/guests/useGuests';
import type { GuestItem } from '@/services/admin/admin.api';

export default function GuestsView() {
  const view = useGuests();

  const columns: TableColumn<GuestItem>[] = [
    {
      key: 'guest',
      header: view.labels.guest,
      cell: (row) => (
        <div className="min-w-0">
          <p className="truncate font-medium text-foreground">{row.fullName || row.userId}</p>
          <p className="truncate text-label text-muted-foreground">{row.contactEmail || row.contactPhone}</p>
        </div>
      ),
    },
    {
      key: 'bookings',
      header: view.labels.bookings,
      align: 'right',
      cell: (row) => <span className="tabular-nums">{row.bookings}</span>,
    },
    {
      key: 'spent',
      header: view.labels.spent,
      align: 'right',
      cell: (row) => <span className="tabular-nums">{row.totalSpent}</span>,
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
            <div className="w-full sm:w-[170px]">
              <Select
                options={[
                  { value: 'cases', label: view.labels.cases },
                  { value: 'cancellations', label: view.labels.cancellations },
                ]}
                value={view.flag}
                onValueChange={view.setFlag}
                placeholder={view.labels.all}
                ariaLabel={view.labels.title}
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
          getRowId={(row) => row.userId}
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
        title={view.selected?.fullName || view.selected?.userId || ''}
        description={view.selected?.contactEmail || view.selected?.contactPhone || ''}
      >
        {view.selected ? (
          <dl className="grid grid-cols-2 gap-3 text-body">
            <div>
              <dt className="text-label text-muted-foreground">{view.labels.bookings}</dt>
              <dd className="tabular-nums">{view.selected.bookings}</dd>
            </div>
            <div>
              <dt className="text-label text-muted-foreground">{view.labels.active}</dt>
              <dd className="tabular-nums">{view.selected.activeBookings}</dd>
            </div>
            <div>
              <dt className="text-label text-muted-foreground">{view.labels.cancelled}</dt>
              <dd className="tabular-nums">{view.selected.cancelledBookings}</dd>
            </div>
            <div>
              <dt className="text-label text-muted-foreground">{view.labels.spent}</dt>
              <dd className="tabular-nums">{view.selected.totalSpent}</dd>
            </div>
          </dl>
        ) : null}
      </Dialog>
    </>
  );
}
