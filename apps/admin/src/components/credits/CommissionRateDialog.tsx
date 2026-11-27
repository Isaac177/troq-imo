import { Button, Dialog, Input, Select, Textarea } from '@mungini/ui-web';
import { useCommissionRateDialog } from '@/hooks/credits/useCommissionRateDialog';

type Props = { hostId: string | null; currentBps: number; open: boolean; onOpenChange: (open: boolean) => void; onSaved: () => void };

export default function CommissionRateDialog({ hostId, currentBps, open, onOpenChange, onSaved }: Props) {
  const view = useCommissionRateDialog(hostId, currentBps, open, () => onOpenChange(false), onSaved);
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
        <p className="text-label text-muted-foreground">{view.labels.current} <span className="tabular-nums text-foreground">{view.currentText}</span></p>
        <label className="flex flex-col gap-1 text-label text-muted-foreground">
          {view.labels.value}
          <Input inputMode="decimal" maxLength={6} value={view.rate} onChange={(event) => view.setRate(event.target.value)} aria-label={view.labels.value} aria-invalid={view.unchanged || undefined} />
          <span className={view.unchanged ? 'text-micro text-tone-attention' : 'text-micro'}>{view.unchanged ? view.labels.unchanged : view.labels.valueHint}</span>
        </label>
        <label className="flex flex-col gap-1 text-label text-muted-foreground">
          {view.labels.reason}
          <Select options={view.reasonOptions} value={view.reasonCode} onValueChange={view.setReasonCode} placeholder={view.labels.chooseReason} ariaLabel={view.labels.reason} />
        </label>
        <label className="flex flex-col gap-1 text-label text-muted-foreground">
          {view.labels.note}
          <Textarea rows={3} maxLength={500} value={view.note} onChange={(event) => view.setNote(event.target.value)} aria-label={view.labels.note} />
          <span className="text-micro">{view.labels.noteHint}</span>
        </label>
      </div>
    </Dialog>
  );
}
