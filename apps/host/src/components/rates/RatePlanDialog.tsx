import { AmountInput, Button, Dialog, Select } from '@mungini/ui-web';
import type { RatePlansView } from '@/hooks/rates/useRatePlans';

type Props = { view: RatePlansView };

export default function RatePlanDialog({ view }: Props) {
  return (
    <>
      <Dialog
        open={view.createOpen}
        onOpenChange={view.setCreateOpen}
        title={view.labels.newPlan}
        footer={
          <div className="flex justify-end gap-2">
            <Button variant="outline" disabled={view.busy} onClick={() => view.setCreateOpen(false)}>{view.labels.close}</Button>
            <Button variant="primary" loading={view.busy} disabled={!view.unitTypeId || !view.cancelPolicyId || !view.payPolicyId} onClick={() => void view.submitCreate()}>{view.labels.save}</Button>
          </div>
        }
      >
        <div className="flex flex-col gap-3">
          {view.formError ? <p role="alert" className="text-label text-destructive">{view.formError}</p> : null}
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.unitType}
            <Select options={view.unitOptions} value={view.unitTypeId} onValueChange={view.setUnitTypeId} ariaLabel={view.labels.unitType} />
          </label>
          <div className="grid grid-cols-1 gap-3 sm:grid-cols-2">
            <label className="flex flex-col gap-1 text-label text-muted-foreground">
              {view.labels.occupancyBasis}
              <Select options={view.occupancyOptions} value={view.occupancyBasis} onValueChange={view.setOccupancyBasis} ariaLabel={view.labels.occupancyBasis} />
            </label>
            <label className="flex flex-col gap-1 text-label text-muted-foreground">
              {view.labels.mealPlan}
              <Select options={view.mealOptions} value={view.mealPlan} onValueChange={view.setMealPlan} ariaLabel={view.labels.mealPlan} />
            </label>
            <label className="flex flex-col gap-1 text-label text-muted-foreground">
              {view.labels.bookingMode}
              <Select options={view.bookingOptions} value={view.bookingMode} onValueChange={view.setBookingMode} ariaLabel={view.labels.bookingMode} />
            </label>
            <label className="flex flex-col gap-1 text-label text-muted-foreground">
              {view.labels.childAmount}
              <AmountInput value={view.childAmount} onValueChange={view.setChildAmount} fractionDigits={2} ariaLabel={view.labels.childAmount} />
            </label>
          </div>
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.cancellationPolicy}
            <Select options={view.cancelOptions} value={view.cancelPolicyId} onValueChange={view.setCancelPolicyId} ariaLabel={view.labels.cancellationPolicy} />
          </label>
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.paymentPolicy}
            <Select options={view.payOptions} value={view.payPolicyId} onValueChange={view.setPayPolicyId} ariaLabel={view.labels.paymentPolicy} />
          </label>
        </div>
      </Dialog>

      <Dialog
        open={view.editTarget !== null}
        onOpenChange={(open) => (open ? undefined : view.setEditTarget(null))}
        title={view.labels.editPlan}
        footer={
          <div className="flex justify-end gap-2">
            <Button variant="outline" disabled={view.busy} onClick={() => view.setEditTarget(null)}>{view.labels.close}</Button>
            <Button variant="primary" loading={view.busy} onClick={() => void view.submitEdit()}>{view.labels.save}</Button>
          </div>
        }
      >
        <div className="flex flex-col gap-3">
          {view.formError ? <p role="alert" className="text-label text-destructive">{view.formError}</p> : null}
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.status}
            <Select options={view.statusOptions} value={view.editStatus} onValueChange={view.setEditStatus} ariaLabel={view.labels.status} />
          </label>
          <div className="grid grid-cols-1 gap-3 sm:grid-cols-2">
            <label className="flex flex-col gap-1 text-label text-muted-foreground">
              {view.labels.bookingMode}
              <Select options={view.bookingOptions} value={view.editBookingMode} onValueChange={view.setEditBookingMode} ariaLabel={view.labels.bookingMode} />
            </label>
            <label className="flex flex-col gap-1 text-label text-muted-foreground">
              {view.labels.mealPlan}
              <Select options={view.mealOptions} value={view.editMealPlan} onValueChange={view.setEditMealPlan} ariaLabel={view.labels.mealPlan} />
            </label>
          </div>
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.childAmount}
            <AmountInput value={view.editChildAmount} onValueChange={view.setEditChildAmount} fractionDigits={2} ariaLabel={view.labels.childAmount} />
          </label>
        </div>
      </Dialog>
    </>
  );
}
