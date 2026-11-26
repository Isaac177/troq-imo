import { Search } from 'lucide-react';
import { EmptyState, Input, PagedTable, Section, Select } from '@mungini/ui-web';
import ReservationDetails from '@/components/reservations/ReservationDetails';
import { useReservations } from '@/hooks/reservations/useReservations';

export default function ReservationsView() {
  const view = useReservations();
  return (
    <>
    <Section title={view.labels.title} description={view.labels.subtitle} actions={<div className="flex w-full flex-col gap-2 sm:w-auto sm:flex-row"><Select options={view.stateOptions} value={view.state} onValueChange={view.setState} ariaLabel={view.statusLabel} /><Input type="search" value={view.search} onChange={event => view.setSearch(event.target.value)} placeholder={view.labels.search} aria-label={view.labels.search} leadingIcon={<Search />} /></div>}>
      {view.error ? <p className="text-label text-destructive">{view.labels.error}</p> : null}
      <PagedTable rows={view.rows} columns={view.columns} getRowId={row => row.id} page={view.page} pageSize={view.pageSize} total={view.total} onPageChange={view.setPage} ariaLabel={view.labels.title} loading={view.loading} onRowClick={row => view.setSelected(row.id)} emptyState={<EmptyState title={view.labels.empty} />} />
    </Section>
    <ReservationDetails id={view.selected} onClose={() => view.setSelected(null)} />
    </>
  );
}
