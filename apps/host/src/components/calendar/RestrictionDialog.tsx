import { Button, ConfirmDialog, DateRangePicker, Dialog, Input, Select } from '@mungini/ui-web';
import type { CalendarRestrictionsView } from '@/hooks/calendar/useCalendarRestrictions';

type Props = { view: CalendarRestrictionsView };

export default function RestrictionDialog({ view }: Props) {
  return (
    <>
      <Dialog
        open={view.open}
        onOpenChange={view.setOpen}
        title={view.labels.addRestriction}
        footer={
          <div className="flex justify-end gap-2">
            <Button variant="outline" disabled={view.busy} onClick={() => view.setOpen(false)}>{view.labels.dismiss}</Button>
            <Button variant="primary" loading={view.busy} disabled={view.numeric && !view.value.trim()} onClick={() => void view.submit()}>{view.labels.addRestriction}</Button>
          </div>
        }
      >
        <div className="flex flex-col gap-3">
          {view.error ? <p role="alert" className="text-label text-destructive">{view.error}</p> : null}
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.restrictionKind}
            <Select options={view.kindOptions} value={view.kind} onValueChange={view.setKind} ariaLabel={view.labels.restrictionKind} />
          </label>
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.restrictionBasis}
            <Select options={view.basisOptions} value={view.basis} onValueChange={view.setBasis} ariaLabel={view.labels.restrictionBasis} />
          </label>
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.restrictionUnit}
            <Select options={view.unitOptions} value={view.unitTypeId} onValueChange={view.setUnitTypeId} ariaLabel={view.labels.restrictionUnit} />
          </label>
          {view.numeric ? (
            <label className="flex flex-col gap-1 text-label text-muted-foreground">
              {view.labels.restrictionValue}
              <Input inputMode="numeric" value={view.value} onChange={(event) => view.setValue(event.target.value)} aria-label={view.labels.restrictionValue} />
            </label>
          ) : null}
          <div className="flex flex-col gap-1 text-label text-muted-foreground">
            <span>{view.labels.addRestriction}</span>
            <DateRangePicker value={view.range} onChange={view.setRange} ariaLabel={view.labels.addRestriction} clearable />
          </div>
        </div>
      </Dialog>

      <ConfirmDialog
        open={view.removeTarget !== null}
        onOpenChange={(open) => (open ? undefined : view.setRemoveTarget(null))}
        title={view.labels.confirmRemoveTitle}
        description={view.labels.confirmRemoveBody}
        confirmLabel={view.labels.removeRestriction}
        cancelLabel={view.labels.dismiss}
        destructive
        loading={view.busy}
        onConfirm={() => void view.confirmRemove()}
      />
    </>
  );
}
