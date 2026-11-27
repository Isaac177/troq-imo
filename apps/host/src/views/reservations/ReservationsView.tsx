import { Link } from 'react-router-dom';
import { Download, Search } from 'lucide-react';
import { Button, DateRangePicker, EmptyState, Input, PagedTable, Section, Select } from '@mungini/ui-web';
import ReservationDrawer from '@/components/reservations/ReservationDrawer';
import { useHostReservations } from '@/hooks/reservations/useHostReservations';
import { useReservationsExport } from '@/hooks/reservations/useReservationsExport';

export default function ReservationsView() {
  const view = useHostReservations();
  const exporter = useReservationsExport({
    hostId: view.hostId,
    propertyId: view.propertyId,
    state: view.state,
    search: view.search,
    checkInFrom: view.range.start,
    checkInTo: view.range.end,
  });

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
              <DateRangePicker value={view.range} onChange={view.setRange} ariaLabel={view.labels.checkInRange} placeholder={view.labels.checkInRange} clearable />
            </div>
            <div className="w-full sm:w-[220px]">
              <Input type="search" value={view.search} onChange={(event) => view.setSearch(event.target.value)} placeholder={view.labels.search} aria-label={view.labels.search} leadingIcon={<Search />} />
            </div>
            {exporter.canExport ? (
              <Button
                variant="outline"
                size="sm"
                icon={<Download />}
                collapseLabel
                aria-label={exporter.labels.export}
                title={exporter.labels.export}
                loading={exporter.exporting}
                disabled={exporter.exporting}
                onClick={exporter.run}
              >
                {exporter.exporting ? exporter.labels.exporting : exporter.labels.export}
              </Button>
            ) : null}
          </div>
        }
      >
        {view.error ? <p role="alert" className="text-label text-destructive">{view.error}</p> : null}
        {exporter.error ? <p role="alert" className="text-label text-destructive">{exporter.error}</p> : null}
        {exporter.notice ? <p role="status" className="text-label text-muted-foreground">{exporter.notice}</p> : null}
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
