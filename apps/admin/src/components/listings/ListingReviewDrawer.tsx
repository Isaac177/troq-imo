import { Button, Checkbox, Dialog, Drawer, Skeleton, Textarea } from '@mungini/ui-web';
import { useListingReview } from '@/hooks/listings/useListingReview';

type Props = { revisionId: string | null; onClose: () => void; onChanged: () => void };

export default function ListingReviewDrawer({ revisionId, onClose, onChanged }: Props) {
  const view = useListingReview(revisionId, onChanged, onClose);
  const revision = view.detail?.revision;
  const decisionTitle = view.decision === 'APPROVE' ? view.labels.approve : view.decision === 'REQUEST_CHANGES' ? view.labels.requestChanges : view.labels.reject;

  return (
    <>
      <Drawer open={revisionId !== null} onOpenChange={(open) => (open ? undefined : onClose())} title={view.labels.review} description={view.reviewStatus} footer={<Button variant="outline" onClick={onClose}>{view.labels.close}</Button>}>
        {view.loading && !revision ? <div className="space-y-2"><Skeleton shape="line" /><Skeleton shape="block" /></div> : null}
        {view.error ? <p role="alert" className="text-label text-destructive">{view.error}</p> : null}
        {revision ? (
          <div className="flex flex-col gap-4">
            <div className="flex flex-wrap gap-2">
              <Button size="sm" variant="primary" onClick={() => view.openDecision('APPROVE')}>{view.labels.approve}</Button>
              <Button size="sm" variant="outline" onClick={() => view.openDecision('REQUEST_CHANGES')}>{view.labels.requestChanges}</Button>
              <Button size="sm" variant="destructive" onClick={() => view.openDecision('REJECT')}>{view.labels.reject}</Button>
            </div>

            <ul className="divide-y divide-border">
              {view.fields.map((field) => (
                <li key={field.key} className="py-3 text-body">
                  <div className="flex items-center justify-between gap-2">
                    <p className="text-label text-muted-foreground">{field.label}</p>
                    <span className={field.changed ? 'text-label text-foreground' : 'text-label text-muted-foreground'}>{field.changed ? view.labels.changed : view.labels.unchanged}</span>
                  </div>
                  <div className="mt-1 grid grid-cols-1 gap-2 sm:grid-cols-2">
                    <div><p className="text-micro text-muted-foreground">{view.labels.live}</p><p className="whitespace-pre-wrap break-words">{field.live || '—'}</p></div>
                    <div><p className="text-micro text-muted-foreground">{view.labels.proposed}</p><p className={field.changed ? 'whitespace-pre-wrap break-words font-medium text-foreground' : 'whitespace-pre-wrap break-words'}>{field.proposed || '—'}</p></div>
                  </div>
                </li>
              ))}
            </ul>
          </div>
        ) : null}
      </Drawer>

      <Dialog
        open={view.decision !== null}
        onOpenChange={(open) => (open ? undefined : view.setDecision(null))}
        title={decisionTitle}
        footer={
          <div className="flex justify-end gap-2">
            <Button variant="outline" disabled={view.busy} onClick={() => view.setDecision(null)}>{view.labels.close}</Button>
            <Button variant={view.decision === 'REJECT' ? 'destructive' : 'primary'} loading={view.busy} onClick={() => void view.submit()}>{view.labels.save}</Button>
          </div>
        }
      >
        <div className="flex flex-col gap-3">
          {view.formError ? <p role="alert" className="text-label text-destructive">{view.formError}</p> : null}
          {view.blockerLabels.length > 0 ? (
            <div>
              <p className="text-label text-muted-foreground">{view.labels.blockers}</p>
              <ul className="mt-1 list-inside list-disc text-body">
                {view.blockerLabels.map((label) => <li key={label}>{label}</li>)}
              </ul>
            </div>
          ) : null}
          {view.decision !== 'APPROVE' ? (
            <div>
              <p className="text-label text-muted-foreground">{view.labels.reasons}</p>
              <div className="mt-1 grid grid-cols-1 gap-1 sm:grid-cols-2">
                {view.reasonOptions.map((reason) => (
                  <Checkbox key={reason.value} checked={view.reasonCodes.includes(reason.value)} onCheckedChange={() => view.toggleReason(reason.value)} label={reason.label} />
                ))}
              </div>
            </div>
          ) : null}
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.guidance}
            <Textarea rows={3} maxLength={1000} value={view.guidance} placeholder={view.labels.guidancePlaceholder} onChange={(event) => view.setGuidance(event.target.value)} />
          </label>
        </div>
      </Dialog>
    </>
  );
}
