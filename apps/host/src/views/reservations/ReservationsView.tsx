import { Link } from 'react-router-dom';
import { Search } from 'lucide-react';
import { Button, EmptyState, Input, PagedTable, Section, Select } from '@mungini/ui-web';
import ReservationDrawer from '@/components/reservations/ReservationDrawer';
import { useHostReservations } from '@/hooks/reservations/useHostReservations';

export default function ReservationsView() {
  const view = useHostReservations();

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
          <div className="flex w-full flex-col gap-2 sm:w-auto sm:flex-row">
            {view.hosts.length > 1 ? (
              <div className="w-full sm:w-[160px]">
                <Select options={view.hosts.map((host) => ({ value: host.hostId, label: host.displayName }))} value={view.hostId} onValueChange={view.setHostId} ariaLabel={view.labels.title} />
              </div>
            ) : null}
            <div className="w-full sm:w-[180px]">
              <Select options={view.propertyOptions} value={view.propertyId} onValueChange={view.setPropertyId} ariaLabel={view.labels.propertyFilter} />
            </div>
            <div className="w-full sm:w-[180px]">
              <Select options={view.stateOptions} value={view.state} onValueChange={view.setState} ariaLabel={view.labels.statusFilter} />
            </div>
            <div className="w-full sm:w-[220px]">
              <Input type="search" value={view.search} onChange={(event) => view.setSearch(event.target.value)} placeholder={view.labels.search} aria-label={view.labels.search} leadingIcon={<Search />} />
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
      <ReservationDrawer reservationId={view.selected} onClose={() => view.setSelected(null)} onChanged={view.refresh} />
    </>
  );
}
