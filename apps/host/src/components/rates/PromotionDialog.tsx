import { AmountInput, Button, DatePicker, Dialog, Input, Select } from '@mungini/ui-web';
import type { PromotionsView } from '@/hooks/rates/usePromotions';

type Props = { view: PromotionsView };

export default function PromotionDialog({ view }: Props) {
  return (
    <Dialog
      open={view.createOpen}
      onOpenChange={view.setCreateOpen}
      title={view.labels.newPromotion}
      footer={
        <div className="flex justify-end gap-2">
          <Button variant="outline" disabled={view.busy} onClick={() => view.setCreateOpen(false)}>{view.labels.close}</Button>
          <Button variant="primary" loading={view.busy} disabled={!view.discountPercent.trim()} onClick={() => void view.submit()}>{view.labels.save}</Button>
        </div>
      }
    >
      <div className="flex flex-col gap-3">
        {view.formError ? <p role="alert" className="text-label text-destructive">{view.formError}</p> : null}
        <label className="flex flex-col gap-1 text-label text-muted-foreground">
          {view.labels.ratePlan}
          <Select options={view.planOptions} value={view.ratePlanId} onValueChange={view.setRatePlanId} ariaLabel={view.labels.ratePlan} />
        </label>
        <div className="grid grid-cols-1 gap-3 sm:grid-cols-2">
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.discount}
            <Input inputMode="decimal" value={view.discountPercent} onChange={(event) => view.setDiscountPercent(event.target.value)} aria-label={view.labels.discount} />
          </label>
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.minNights}
            <Input inputMode="numeric" value={view.minNights} onChange={(event) => view.setMinNights(event.target.value)} aria-label={view.labels.minNights} />
          </label>
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.capAmount}
            <AmountInput value={view.capAmount} onValueChange={view.setCapAmount} fractionDigits={2} ariaLabel={view.labels.capAmount} />
          </label>
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.redemptions}
            <Input inputMode="numeric" value={view.redemptionLimit} onChange={(event) => view.setRedemptionLimit(event.target.value)} aria-label={view.labels.redemptions} />
          </label>
          <div className="flex flex-col gap-1 text-label text-muted-foreground">
            <span>{view.labels.startDate}</span>
            <DatePicker value={view.startDate} onChange={view.setStartDate} ariaLabel={view.labels.startDate} clearable />
          </div>
          <div className="flex flex-col gap-1 text-label text-muted-foreground">
            <span>{view.labels.endDate}</span>
            <DatePicker value={view.endDate} onChange={view.setEndDate} ariaLabel={view.labels.endDate} clearable />
          </div>
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.priority}
            <Input inputMode="numeric" value={view.priority} onChange={(event) => view.setPriority(event.target.value)} aria-label={view.labels.priority} />
          </label>
        </div>
      </div>
    </Dialog>
  );
}
