import { AmountInput, Button, Dialog, Input, Select, Textarea } from '@mungini/ui-web';
import type { CreditEntryMode } from '@/types/credits/credits';
import { useCreditEntryDialog } from '@/hooks/credits/useCreditEntryDialog';

type Props = { hostId: string | null; mode: CreditEntryMode; open: boolean; onOpenChange: (open: boolean) => void; onSaved: () => void };

export default function CreditEntryDialog({ hostId, mode, open, onOpenChange, onSaved }: Props) {
  const view = useCreditEntryDialog(hostId, mode, open, () => onOpenChange(false), onSaved);
  return (
    <Dialog
      open={open}
      onOpenChange={onOpenChange}
      title={view.labels.title}
      description={view.labels.description}
      size="sm"
      footer={
        <div className="flex justify-end gap-2">
          <Button variant="outline" disabled={view.busy} onClick={() => onOpenChange(false)}>{view.labels.cancel}</Button>
          <Button variant="primary" loading={view.busy} disabled={!view.canSubmit} onClick={() => void view.submit()}>{view.labels.submit}</Button>
        </div>
      }
    >
      <div className="flex flex-col gap-3">
        {view.formError ? <p role="alert" className="text-label text-destructive">{view.formError}</p> : null}
        {mode === 'adjust' ? (
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.direction}
            <Select options={view.directionOptions} value={view.direction} onValueChange={view.setDirection} ariaLabel={view.labels.direction} />
          </label>
        ) : null}
        <label className="flex flex-col gap-1 text-label text-muted-foreground">
          {view.labels.amount}
          <AmountInput value={view.amount} onValueChange={view.setAmount} fractionDigits={2} ariaLabel={view.labels.amount} />
          <span className="text-micro">{view.labels.amountHint}</span>
        </label>
        <label className="flex flex-col gap-1 text-label text-muted-foreground">
          {view.labels.reason}
          <Select options={view.reasonOptions} value={view.reasonCode} onValueChange={view.setReasonCode} placeholder={view.labels.chooseReason} ariaLabel={view.labels.reason} />
        </label>
        {view.showReference ? (
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.paymentRef}
            <Input value={view.paymentRef} maxLength={120} onChange={(event) => view.setPaymentRef(event.target.value)} aria-label={view.labels.paymentRef} />
            <span className="text-micro">{view.labels.paymentRefHint}</span>
          </label>
        ) : null}
        <label className="flex flex-col gap-1 text-label text-muted-foreground">
          {view.labels.note}
          <Textarea rows={3} maxLength={500} value={view.note} onChange={(event) => view.setNote(event.target.value)} aria-label={view.labels.note} />
          <span className="text-micro">{view.labels.noteHint}</span>
        </label>
      </div>
    </Dialog>
  );
}
