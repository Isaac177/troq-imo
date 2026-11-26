import { Button, Dialog, Select } from '@mungini/ui-web';
import type { AdminReviewsView } from '@/hooks/reviews/useAdminReviews';

type Props = { view: AdminReviewsView };

export default function ReviewModerationDialog({ view }: Props) {
  return (
    <Dialog
      open={view.target !== null}
      onOpenChange={(open) => (open ? undefined : view.setTarget(null))}
      title={view.labels.moderationTitle}
      description={view.labels.moderationBody}
      footer={
        <div className="flex justify-end gap-2">
          <Button variant="outline" disabled={view.busy} onClick={() => view.setTarget(null)}>{view.labels.close}</Button>
          <Button variant={view.decision === 'HIDDEN' ? 'destructive' : 'primary'} loading={view.busy} disabled={view.decision === 'HIDDEN' && !view.reason} onClick={() => void view.submit()}>{view.labels.save}</Button>
        </div>
      }
    >
      <div className="flex flex-col gap-3">
        {view.formError ? <p role="alert" className="text-label text-destructive">{view.formError}</p> : null}
        {view.selectedComment ? (
          <div className="border-t border-border pt-2">
            <p className="text-label text-muted-foreground">{view.labels.comment}</p>
            <p className="mt-1 whitespace-pre-wrap text-body text-foreground">{view.selectedComment}</p>
          </div>
        ) : null}
        {view.selectedReply ? (
          <div className="border-t border-border pt-2">
            <p className="text-label text-muted-foreground">{view.labels.reply}</p>
            <p className="mt-1 whitespace-pre-wrap text-body text-foreground">{view.selectedReply}</p>
          </div>
        ) : null}
        <label className="flex flex-col gap-1 text-label text-muted-foreground">
          {view.labels.decision}
          <Select options={view.decisionOptions} value={view.decision} onValueChange={view.setDecision} ariaLabel={view.labels.decision} />
        </label>
        {view.decision === 'HIDDEN' ? (
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.reason}
            <Select options={view.reasonOptions} value={view.reason} onValueChange={view.setReason} placeholder={view.labels.chooseReason} ariaLabel={view.labels.reason} />
          </label>
        ) : null}
      </div>
    </Dialog>
  );
}
