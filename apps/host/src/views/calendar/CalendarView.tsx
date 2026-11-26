import { Link } from 'react-router-dom';
import { Ban } from 'lucide-react';
import { Button, DateRangePicker, EmptyState, Section, Select, Skeleton } from '@mungini/ui-web';
import BlockDatesDialog from '@/components/calendar/BlockDatesDialog';
import CalendarBlockList from '@/components/calendar/CalendarBlockList';
import CalendarReservationList from '@/components/calendar/CalendarReservationList';
import CalendarRestrictionList from '@/components/calendar/CalendarRestrictionList';
import UnitStrip from '@/components/calendar/UnitStrip';
import ReservationDrawer from '@/components/reservations/ReservationDrawer';
import { useCalendarBlocks } from '@/hooks/calendar/useCalendarBlocks';
import { useHostCalendar } from '@/hooks/calendar/useHostCalendar';

export default function CalendarView() {
  const view = useHostCalendar();
  const blocks = useCalendarBlocks(view.propertyId, view.unitOptions, view.refresh);

  if (!view.hasHost) {
    return (
      <Section title={view.labels.title} description={view.labels.subtitle}>
        <EmptyState title={view.labels.noHost} action={<Button asChild variant="primary" size="sm"><Link to="/onboarding">{view.labels.noHostAction}</Link></Button>} />
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
            <div className="w-full sm:w-[200px]">
              <Select options={view.properties} value={view.propertyId} onValueChange={view.setPropertyId} placeholder={view.labels.chooseProperty} ariaLabel={view.labels.property} />
            </div>
            <div className="w-full sm:w-[210px]">
              <DateRangePicker value={view.range} onChange={view.setRange} ariaLabel={view.labels.range} />
            </div>
            {view.canWrite && view.propertyId ? (
              <Button variant="primary" size="sm" icon={<Ban />} collapseLabel aria-label={view.labels.blockDates} title={view.labels.blockDates} onClick={() => blocks.setOpen(true)}>{view.labels.blockDates}</Button>
            ) : null}
          </div>
        }
      >
        {!view.validRange ? <p role="alert" className="text-label text-destructive">{view.labels.invalidRange}</p> : null}
        {view.error ? <p role="alert" className="text-label text-destructive">{view.error}</p> : null}
        {blocks.notice ? <p role="status" className="text-label text-muted-foreground">{blocks.notice}</p> : null}
        {view.loading ? <div className="space-y-2"><Skeleton shape="line" /><Skeleton shape="block" /></div> : null}

        {!view.loading && view.propertyId ? (
          <div className="flex flex-col gap-5">
            <div className="flex flex-col gap-3">
              <div className="flex flex-wrap items-center justify-between gap-2">
                <h2 className="text-title font-semibold text-foreground">{view.labels.units}</h2>
                <p className="text-label text-muted-foreground">{view.labels.allocated} / {view.labels.capacity}</p>
              </div>
              {view.strips.length === 0 ? <p className="text-body text-muted-foreground">{view.labels.unitsEmpty}</p> : null}
              {view.strips.map((strip) => <UnitStrip key={strip.unitTypeId} name={strip.name} days={strip.days} labels={{ open: view.labels.open, full: view.labels.full }} />)}
              {view.unitPages > 1 ? (
                <div className="flex items-center gap-2 text-label text-muted-foreground">
                  <Button variant="ghost" size="sm" disabled={view.page <= 1} onClick={() => view.setPage(view.page - 1)}>{view.labels.previous}</Button>
                  <span>{view.page} / {view.unitPages}</span>
                  <Button variant="ghost" size="sm" disabled={view.page >= view.unitPages} onClick={() => view.setPage(view.page + 1)}>{view.labels.next}</Button>
                </div>
              ) : null}
            </div>

            <div className="border-t border-border pt-4">
              <h2 className="text-title font-semibold text-foreground">{view.labels.reservations}</h2>
              <div className="mt-2">
                <CalendarReservationList rows={view.reservations} empty={view.labels.noReservations} guestLabel={view.labels.guest} onOpen={view.setSelected} />
              </div>
            </div>

            <div className="border-t border-border pt-4">
              <h2 className="text-title font-semibold text-foreground">{view.labels.blocks}</h2>
              <div className="mt-2">
                <CalendarBlockList rows={view.blocks} empty={view.labels.noBlocks} removeLabel={view.labels.removeBlock} onRemove={blocks.askRemove} />
              </div>
            </div>

            <div className="border-t border-border pt-4">
              <h2 className="text-title font-semibold text-foreground">{view.labels.restrictions}</h2>
              <div className="mt-2">
                <CalendarRestrictionList rows={view.restrictions} empty={view.labels.noRestrictions} />
              </div>
            </div>
          </div>
        ) : null}
      </Section>

      <BlockDatesDialog view={blocks} unitOptions={view.unitOptions} />
      <ReservationDrawer reservationId={view.selected} onClose={() => view.setSelected(null)} onChanged={view.refresh} />
    </>
  );
}
