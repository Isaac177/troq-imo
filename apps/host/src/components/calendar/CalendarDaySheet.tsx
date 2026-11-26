import { Button, SlideUpDrawer } from '@mungini/ui-web';
import CalendarBlockList from '@/components/calendar/CalendarBlockList';
import CalendarReservationList from '@/components/calendar/CalendarReservationList';
import CalendarRestrictionList from '@/components/calendar/CalendarRestrictionList';
import type { CalendarMonthState } from '@/hooks/calendar/useCalendarMonth';

type Props = { view: CalendarMonthState };

export default function CalendarDaySheet({ view }: Props) {
  return (
    <SlideUpDrawer
      open={view.selectedDay !== null}
      onOpenChange={(open) => (open ? undefined : view.closeDay())}
      title={view.sheetTitle}
      description={view.sheetDescription}
      footer={<Button variant="outline" onClick={view.closeDay}>{view.labels.close}</Button>}
    >
      <div className="flex flex-col gap-4">
        <div>
          <h3 className="text-body font-semibold text-foreground">{view.labels.reservations}</h3>
          <div className="mt-1">
            <CalendarReservationList rows={view.reservations} empty={view.labels.noReservations} guestLabel={view.labels.guest} onOpen={view.openReservation} />
          </div>
        </div>
        <div className="border-t border-border pt-3">
          <h3 className="text-body font-semibold text-foreground">{view.labels.blocks}</h3>
          <div className="mt-1">
            <CalendarBlockList rows={view.blocks} empty={view.labels.noBlocks} />
          </div>
        </div>
        <div className="border-t border-border pt-3">
          <h3 className="text-body font-semibold text-foreground">{view.labels.restrictions}</h3>
          <div className="mt-1">
            <CalendarRestrictionList rows={view.restrictions} empty={view.labels.noRestrictions} />
          </div>
        </div>
      </div>
    </SlideUpDrawer>
  );
}
