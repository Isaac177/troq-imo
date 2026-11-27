import { Ban, Plus } from 'lucide-react';
import { Button, SlideUpDrawer } from '@mungini/ui-web';
import CalendarBlockList from '@/components/calendar/CalendarBlockList';
import CalendarReservationList from '@/components/calendar/CalendarReservationList';
import CalendarRestrictionList from '@/components/calendar/CalendarRestrictionList';
import type { CalendarMonthState } from '@/hooks/calendar/useCalendarMonth';
import type { CalendarDayActions } from '@/hooks/calendar/useCalendarDayActions';

type Props = { view: CalendarMonthState; actions: CalendarDayActions };

export default function CalendarDaySheet({ view, actions }: Props) {
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
          <div className="flex items-center justify-between gap-2">
            <h3 className="text-body font-semibold text-foreground">{view.labels.blocks}</h3>
            {actions.canWrite ? <Button size="sm" variant="outline" icon={<Ban />} collapseLabel aria-label={actions.labels.block} title={actions.labels.block} onClick={actions.addBlock}>{actions.labels.block}</Button> : null}
          </div>
          <div className="mt-1">
            <CalendarBlockList rows={view.blocks} empty={view.labels.noBlocks} removeLabel={actions.labels.removeBlock} onRemove={actions.canWrite ? actions.removeBlock : undefined} />
          </div>
        </div>
        <div className="border-t border-border pt-3">
          <div className="flex items-center justify-between gap-2">
            <h3 className="text-body font-semibold text-foreground">{view.labels.restrictions}</h3>
            {actions.canWrite ? <Button size="sm" variant="outline" icon={<Plus />} collapseLabel aria-label={actions.labels.restriction} title={actions.labels.restriction} onClick={actions.addRestriction}>{actions.labels.restriction}</Button> : null}
          </div>
          <div className="mt-1">
            <CalendarRestrictionList rows={view.restrictions} empty={view.labels.noRestrictions} removeLabel={actions.labels.removeRestriction} onRemove={actions.canWrite ? actions.removeRestriction : undefined} />
          </div>
        </div>
      </div>
    </SlideUpDrawer>
  );
}
