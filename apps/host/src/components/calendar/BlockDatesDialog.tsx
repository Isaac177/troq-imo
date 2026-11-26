import { Button, ConfirmDialog, DateRangePicker, Dialog, Select, Textarea } from '@mungini/ui-web';
import type { CalendarBlocksView } from '@/hooks/calendar/useCalendarBlocks';
import type { PropertyOption } from '@/types/calendar/calendar';

type Props = { view: CalendarBlocksView; unitOptions: PropertyOption[] };

export default function BlockDatesDialog({ view, unitOptions }: Props) {
  return (
    <>
      <Dialog
        open={view.open}
        onOpenChange={view.setOpen}
        title={view.labels.blockTitle}
        description={view.labels.blockBody}
        footer={
          <div className="flex justify-end gap-2">
            <Button variant="outline" disabled={view.busy} onClick={() => view.setOpen(false)}>{view.labels.dismiss}</Button>
            <Button variant="primary" loading={view.busy} disabled={!view.unitTypeId || !view.range.start || !view.range.end || !view.reason.trim()} onClick={() => void view.submit()}>{view.labels.blockSubmit}</Button>
          </div>
        }
      >
        <div className="flex flex-col gap-3">
          {view.error ? <p role="alert" className="text-label text-destructive">{view.error}</p> : null}
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.blockUnit}
            <Select options={unitOptions} value={view.unitTypeId} onValueChange={view.setUnitTypeId} ariaLabel={view.labels.blockUnit} />
          </label>
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.blockReason}
            <Textarea rows={3} maxLength={300} value={view.reason} placeholder={view.labels.blockReasonPlaceholder} onChange={(event) => view.setReason(event.target.value)} />
          </label>
          <div className="flex flex-col gap-1 text-label text-muted-foreground">
            <span>{view.labels.blockTitle}</span>
            <DateRangePicker value={view.range} onChange={view.setRange} ariaLabel={view.labels.blockTitle} />
          </div>
        </div>
      </Dialog>

      <ConfirmDialog
        open={view.removeTarget !== null}
        onOpenChange={(open) => (open ? undefined : view.setRemoveTarget(null))}
        title={view.labels.confirmRemoveTitle}
        description={view.labels.confirmRemoveBody}
        confirmLabel={view.labels.removeBlock}
        cancelLabel={view.labels.dismiss}
        destructive
        loading={view.removeBusy}
        onConfirm={() => void view.confirmRemove()}
      />
    </>
  );
}
