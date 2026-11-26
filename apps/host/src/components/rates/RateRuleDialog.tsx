import { AmountInput, Button, ConfirmDialog, DatePicker, Dialog, Input, Select, cn } from '@mungini/ui-web';
import type { RateRulesView } from '@/hooks/rates/useRateRules';

type Props = { view: RateRulesView; planName: string };

export default function RateRuleDialog({ view, planName }: Props) {
  return (
    <>
      <Dialog
        open={view.createOpen}
        onOpenChange={view.setCreateOpen}
        title={`${view.labels.addRule} · ${planName}`}
        footer={
          <div className="flex justify-end gap-2">
            <Button variant="outline" disabled={view.busy} onClick={() => view.setCreateOpen(false)}>{view.labels.close}</Button>
            <Button variant="primary" loading={view.busy} disabled={view.weekdays.length === 0 || !view.amount.trim()} onClick={() => void view.submit()}>{view.labels.save}</Button>
          </div>
        }
      >
        <div className="flex flex-col gap-3">
          {view.formError ? <p role="alert" className="text-label text-destructive">{view.formError}</p> : null}
          <div className="grid grid-cols-1 gap-3 sm:grid-cols-2">
            <div className="flex flex-col gap-1 text-label text-muted-foreground">
              <span>{view.labels.startDate}</span>
              <DatePicker value={view.startDate} onChange={view.setStartDate} ariaLabel={view.labels.startDate} clearable />
            </div>
            <div className="flex flex-col gap-1 text-label text-muted-foreground">
              <span>{view.labels.endDate}</span>
              <DatePicker value={view.endDate} onChange={view.setEndDate} ariaLabel={view.labels.endDate} clearable />
            </div>
          </div>
          <div className="flex flex-col gap-1">
            <span className="text-label text-muted-foreground">{view.labels.weekdays}</span>
            <div className="grid grid-cols-7 gap-1">
              {view.weekdayOptions.map((day) => (
                <Button
                  key={day.day}
                  type="button"
                  size="sm"
                  variant={view.weekdays.includes(day.day) ? 'secondary' : 'outline'}
                  aria-pressed={view.weekdays.includes(day.day)}
                  onClick={() => view.toggleWeekday(day.day)}
                  className={cn('px-0')}
                >
                  {day.label}
                </Button>
              ))}
            </div>
          </div>
          <div className="grid grid-cols-1 gap-3 sm:grid-cols-2">
            <label className="flex flex-col gap-1 text-label text-muted-foreground">
              {view.labels.adjustment}
              <Select options={view.adjustmentOptions} value={view.adjustmentType} onValueChange={view.setAdjustmentType} ariaLabel={view.labels.adjustment} />
            </label>
            <label className="flex flex-col gap-1 text-label text-muted-foreground">
              {view.labels.amount}
              {view.adjustmentType === 'PERCENTAGE'
                ? <Input inputMode="decimal" value={view.amount} onChange={(event) => view.setAmount(event.target.value)} aria-label={view.labels.amount} />
                : <AmountInput value={view.amount} onValueChange={view.setAmount} fractionDigits={2} ariaLabel={view.labels.amount} />}
            </label>
            <label className="flex flex-col gap-1 text-label text-muted-foreground">
              {view.labels.priority}
              <Input inputMode="numeric" value={String(view.priority)} onChange={(event) => view.setPriority(Number(event.target.value) || 0)} aria-label={view.labels.priority} />
            </label>
          </div>
        </div>
      </Dialog>

      <ConfirmDialog
        open={view.removeTarget !== null}
        onOpenChange={(open) => (open ? undefined : view.setRemoveTarget(null))}
        title={view.labels.removeRule}
        description={planName}
        confirmLabel={view.labels.removeRule}
        cancelLabel={view.labels.cancel}
        destructive
        loading={view.busy}
        onConfirm={() => void view.confirmRemove()}
      />
    </>
  );
}
