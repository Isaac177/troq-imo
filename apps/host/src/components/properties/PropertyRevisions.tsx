import { Plus } from 'lucide-react';
import { Button, ConfirmDialog, Skeleton } from '@mungini/ui-web';
import { usePropertyRevisions } from '@/hooks/properties/usePropertyRevisions';
import RevisionEditorDialog from '@/components/properties/RevisionEditorDialog';

type Props = { propertyId: string; onChanged: () => void };

export default function PropertyRevisions({ propertyId, onChanged }: Props) {
  const view = usePropertyRevisions(propertyId, onChanged);

  return (
    <div className="border-t border-border pt-3">
      <div className="flex items-center justify-between gap-2">
        <p className="text-body font-semibold text-foreground">{view.labels.revisions}</p>
        <Button size="sm" variant="outline" icon={<Plus />} collapseLabel aria-label={view.labels.newRevision} title={view.labels.newRevision} loading={view.busy} onClick={() => void view.createNew()}>{view.labels.newRevision}</Button>
      </div>
      {view.error ? <p role="alert" className="mt-2 text-label text-destructive">{view.error}</p> : null}
      {view.notice ? <p role="status" className="mt-2 text-label text-muted-foreground">{view.notice}</p> : null}
      {view.loading ? <div className="mt-2 space-y-2"><Skeleton shape="line" /><Skeleton shape="line" /></div> : null}
      {!view.loading && view.rows.length === 0 ? <p className="mt-2 text-label text-muted-foreground">{view.labels.noRevisions}</p> : null}
      <ul className="mt-2 divide-y divide-border">
        {view.rows.map((revision) => (
          <li key={revision.id} className="flex flex-wrap items-center justify-between gap-2 py-2 text-body">
            <div className="min-w-0">
              <p className="truncate font-medium text-foreground">#{revision.revisionNumber} · {view.statusLabel(revision.reviewStatus)}</p>
              <p className="truncate text-label text-muted-foreground">{view.canWithdraw(revision.reviewStatus) ? view.time(revision.submittedAt) : view.time(revision.updatedAt)}</p>
            </div>
            <div className="flex gap-1">
              {view.canEdit(revision.reviewStatus) ? <Button size="sm" variant="outline" onClick={() => void view.open(revision.id)}>{view.labels.openRevision}</Button> : null}
              {view.canWithdraw(revision.reviewStatus) ? <Button size="sm" variant="ghost" onClick={() => view.setWithdrawTarget(revision)}>{view.labels.withdrawRevision}</Button> : null}
            </div>
          </li>
        ))}
      </ul>

      <RevisionEditorDialog propertyId={propertyId} revision={view.editing} onSaved={view.setEditing} onClose={() => view.setEditing(null)} onSubmitted={() => { view.setEditing(null); view.reload(); onChanged(); }} />

      <ConfirmDialog
        open={view.withdrawTarget !== null}
        onOpenChange={(open) => (open ? undefined : view.setWithdrawTarget(null))}
        title={view.labels.confirmWithdrawTitle}
        description={view.labels.confirmWithdrawBody}
        confirmLabel={view.labels.withdrawRevision}
        cancelLabel={view.labels.dismiss}
        destructive
        loading={view.busy}
        onConfirm={() => void view.confirmWithdraw()}
      />
    </div>
  );
}
