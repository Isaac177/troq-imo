import { Link } from 'react-router-dom';
import { Ban } from 'lucide-react';
import { Button, DateRangePicker, EmptyState, FadeIn, SegmentedControl, Section, Select, Skeleton } from '@mungini/ui-web';
import BlockDatesDialog from '@/components/calendar/BlockDatesDialog';
import CalendarBlockList from '@/components/calendar/CalendarBlockList';
import CalendarDaySheet from '@/components/calendar/CalendarDaySheet';
import CalendarMonthView from '@/components/calendar/CalendarMonthView';
import CalendarReservationList from '@/components/calendar/CalendarReservationList';
import CalendarRestrictionList from '@/components/calendar/CalendarRestrictionList';
import CalendarUnitPager from '@/components/calendar/CalendarUnitPager';
import RestrictionDialog from '@/components/calendar/RestrictionDialog';
import UnitStrip from '@/components/calendar/UnitStrip';
import ReservationDrawer from '@/components/reservations/ReservationDrawer';
import { useCalendarBlocks } from '@/hooks/calendar/useCalendarBlocks';
import { useCalendarMonth } from '@/hooks/calendar/useCalendarMonth';
import { useCalendarRestrictions } from '@/hooks/calendar/useCalendarRestrictions';
import { useHostCalendar } from '@/hooks/calendar/useHostCalendar';

export default function CalendarView() {
  const view = useHostCalendar();
  const blocks = useCalendarBlocks(view.propertyId, view.unitOptions, view.refresh);
  const restrictions = useCalendarRestrictions(view.propertyId, view.unitOptions, view.refresh);
  const month = useCalendarMonth(view.calendar, view.range, view.setSelected);

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
              <div className="flex min-w-0 justify-end">
                <SegmentedControl items={view.viewOptions} value={view.mode} onChange={view.setMode} ariaLabel={view.labels.view} />
              </div>
              <FadeIn swapKey={view.mode} className="flex flex-col gap-3">
                {view.mode === 'month' ? (
                  <CalendarMonthView view={month} onPreviousMonth={view.previousMonth} onNextMonth={view.nextMonth} />
                ) : (
                  <>
                    <div className="flex flex-wrap items-center justify-between gap-2">
                      <h2 className="text-title font-semibold text-foreground">{view.labels.units}</h2>
                      <p className="text-label text-muted-foreground">{view.labels.allocated} / {view.labels.capacity}</p>
                    </div>
                    {view.strips.length === 0 ? <p className="text-body text-muted-foreground">{view.labels.unitsEmpty}</p> : null}
                    {view.strips.map((strip) => <UnitStrip key={strip.unitTypeId} name={strip.name} days={strip.days} labels={{ open: view.labels.open, full: view.labels.full }} />)}
                  </>
                )}
              </FadeIn>
              {view.unitPages > 1 ? (
                <CalendarUnitPager
                  page={view.page}
                  pages={view.unitPages}
                  label={view.mode === 'month' ? view.labels.units : undefined}
                  previousLabel={view.labels.previous}
                  nextLabel={view.labels.next}
                  onPrevious={view.previousPage}
                  onNext={view.nextPage}
                />
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
              <div className="flex flex-wrap items-center justify-between gap-2">
                <h2 className="text-title font-semibold text-foreground">{view.labels.restrictions}</h2>
                {view.canWrite && view.propertyId ? <Button variant="outline" size="sm" icon={<Ban />} collapseLabel aria-label={restrictions.labels.addRestriction} title={restrictions.labels.addRestriction} onClick={restrictions.openCreate}>{restrictions.labels.addRestriction}</Button> : null}
              </div>
              {restrictions.error ? <p role="alert" className="mt-2 text-label text-destructive">{restrictions.error}</p> : null}
              {restrictions.notice ? <p role="status" className="mt-2 text-label text-muted-foreground">{restrictions.notice}</p> : null}
              <div className="mt-2">
                <CalendarRestrictionList rows={view.restrictions} empty={view.labels.noRestrictions} removeLabel={restrictions.labels.removeRestriction} onRemove={view.canWrite ? restrictions.setRemoveTarget : undefined} />
              </div>
            </div>
          </div>
        ) : null}
      </Section>

      <BlockDatesDialog view={blocks} unitOptions={view.unitOptions} />
      <RestrictionDialog view={restrictions} />
      <CalendarDaySheet view={month} />
      <ReservationDrawer reservationId={view.selected} onClose={() => view.setSelected(null)} onChanged={view.refresh} />
    </>
  );
}
