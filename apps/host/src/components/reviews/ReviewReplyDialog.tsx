import { Button, Dialog, Textarea } from '@mungini/ui-web';
import type { HostReviewsView } from '@/hooks/reviews/useHostReviews';

type Props = { view: HostReviewsView };

export default function ReviewReplyDialog({ view }: Props) {
  return (
    <Dialog
      open={view.replyTarget !== null}
      onOpenChange={(open) => (open ? undefined : view.setReplyTarget(null))}
      title={view.labels.replyTitle}
      description={view.replyText}
      footer={
        <div className="flex justify-end gap-2">
          <Button variant="outline" disabled={view.busy} onClick={() => view.setReplyTarget(null)}>{view.labels.close}</Button>
          <Button variant="primary" loading={view.busy} disabled={!view.draft.trim()} onClick={() => void view.submit()}>{view.labels.sendReply}</Button>
        </div>
      }
    >
      <div className="flex flex-col gap-2">
        {view.formError ? <p role="alert" className="text-label text-destructive">{view.formError}</p> : null}
        <label className="flex flex-col gap-1 text-label text-muted-foreground">
          {view.labels.comment}
          <Textarea rows={4} maxLength={2000} value={view.draft} placeholder={view.labels.replyPlaceholder} onChange={(event) => view.setDraft(event.target.value)} />
        </label>
      </div>
    </Dialog>
  );
}
