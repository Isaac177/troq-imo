import { KeyRound } from 'lucide-react';
import { Button, DatePicker, Dialog, Select } from '@mungini/ui-web';
import { useReservationAssignment } from '@/hooks/reservations/useReservationAssignment';

type Props = { itemId: string; state: string; assignmentId: string | null; assignedUnitCode: string | null; checkIn: string; enabled: boolean; onChanged: () => void };

export default function ReservationAssignment({ itemId, state, assignmentId, assignedUnitCode, checkIn, enabled, onChanged }: Props) {
  const view = useReservationAssignment({ itemId, state, assignmentId, checkIn }, onChanged);
  if (!enabled || !view.canAssign) return null;
  return (
    <div className="border-t border-border pt-3">
      <div className="flex flex-wrap items-center justify-between gap-2">
        <div className="min-w-0">
          <p className="text-label text-muted-foreground">{view.labels.roomAssignment}</p>
          <p className="text-body font-medium text-foreground">{assignedUnitCode || view.labels.noRoomAssigned}</p>
        </div>
        <Button size="sm" variant="outline" icon={<KeyRound />} onClick={() => void view.openDialog()}>{assignmentId ? view.labels.changeRoom : view.labels.assignRoom}</Button>
      </div>
      {view.notice ? <p role="status" className="mt-2 text-label text-muted-foreground">{view.notice}</p> : null}
      {view.error ? <p role="alert" className="mt-2 text-label text-destructive">{view.error}</p> : null}

      <Dialog
        open={view.open}
        onOpenChange={view.setOpen}
        title={assignmentId ? view.labels.changeRoom : view.labels.assignRoom}
        footer={
          <div className="flex justify-end gap-2">
            <Button variant="outline" disabled={view.busy} onClick={() => view.setOpen(false)}>{view.labels.dismiss}</Button>
            <Button variant="primary" loading={view.busy} disabled={!view.unitId} onClick={() => void view.submit()}>{assignmentId ? view.labels.changeRoom : view.labels.assignRoom}</Button>
          </div>
        }
      >
        <div className="flex flex-col gap-3">
          {view.loading ? <p className="text-body text-muted-foreground">{view.labels.chooseRoom}</p> : null}
          {view.error ? <p role="alert" className="text-label text-destructive">{view.error}</p> : null}
          {!view.loading && view.optionOptions.length === 0 ? <p className="text-body text-muted-foreground">{view.labels.noRoomOptions}</p> : null}
          {view.optionOptions.length > 0 ? (
            <label className="flex flex-col gap-1 text-label text-muted-foreground">
              {view.labels.chooseRoom}
              <Select options={view.optionOptions} value={view.unitId} onValueChange={view.setUnitId} ariaLabel={view.labels.chooseRoom} />
            </label>
          ) : null}
          {assignmentId ? (
            <div className="flex flex-col gap-1 text-label text-muted-foreground">
              <span>{view.labels.moveFrom}</span>
              <DatePicker value={view.fromDate} onChange={(value) => view.setFromDate(value ?? checkIn)} ariaLabel={view.labels.moveFrom} />
            </div>
          ) : null}
        </div>
      </Dialog>
    </div>
  );
}
