import { CheckCircle2, FileText, TriangleAlert } from 'lucide-react';
import { Button, Dialog, Drawer, Skeleton, Textarea } from '@mungini/ui-web';
import { useCaseDetail } from '@/hooks/cases/useCaseDetail';

type Props = { caseId: string | null; onClose: () => void; onChanged: () => void };

export default function CaseDetailDrawer({ caseId, onClose, onChanged }: Props) {
  const view = useCaseDetail(caseId, onChanged);
  const item = view.detail?.item;
  const isImage = view.preview?.mimeType.startsWith('image/') ?? false;
  const isPdf = view.preview?.mimeType === 'application/pdf';

  return (
    <>
      <Drawer open={caseId !== null} onOpenChange={(open) => (open ? undefined : onClose())} title={item ? view.typeLabel(item.caseType) : view.labels.loading} description={item?.reservationReference ?? ''} footer={<Button variant="outline" onClick={onClose}>{view.labels.close}</Button>}>
        {view.loading && !item ? <div className="space-y-2"><Skeleton shape="line" /><Skeleton shape="line" /><Skeleton shape="block" /></div> : null}
        {view.error ? <p role="alert" className="text-label text-destructive">{view.labels.error}</p> : null}
        {view.notice ? <p role="status" className="text-label text-muted-foreground">{view.notice}</p> : null}
        {item ? (
          <div className="flex flex-col gap-4">
            <dl className="grid grid-cols-1 gap-3 text-body sm:grid-cols-2">
              <div><dt className="text-label text-muted-foreground">{view.labels.state}</dt><dd>{view.stateLabel(item.state)}</dd></div>
              <div><dt className="text-label text-muted-foreground">{view.labels.operator}</dt><dd>{item.operatorName}</dd></div>
              <div><dt className="text-label text-muted-foreground">{view.labels.reservation}</dt><dd>{item.reservationReference ?? '—'}</dd></div>
              <div><dt className="text-label text-muted-foreground">{view.labels.claimAmount}</dt><dd className="tabular-nums">{view.money(item.claimAmount)}</dd></div>
              <div><dt className="text-label text-muted-foreground">{view.labels.createdAt}</dt><dd className="tabular-nums">{view.time(item.createdAt)}</dd></div>
              <div><dt className="text-label text-muted-foreground">{view.labels.deadline}</dt><dd className={item.isOverdue ? 'tabular-nums text-destructive' : 'tabular-nums'}>{view.time(item.deadlineAt)}</dd></div>
            </dl>

            <div className="border-t border-border pt-3">
              <p className="text-label text-muted-foreground">{view.labels.reason}</p>
              <p className="mt-1 whitespace-pre-wrap text-body text-foreground">{item.reason}</p>
            </div>

            {item.escalationReason ? (
              <div className="border-t border-border pt-3">
                <p className="text-label text-muted-foreground">{view.labels.escalationReason}</p>
                <p className="mt-1 whitespace-pre-wrap text-body text-foreground">{item.escalationReason}</p>
              </div>
            ) : null}
            {item.resolution ? (
              <div className="border-t border-border pt-3">
                <p className="text-label text-muted-foreground">{view.labels.resolution}</p>
                <p className="mt-1 whitespace-pre-wrap text-body text-foreground">{typeof item.resolution === 'object' ? JSON.stringify(item.resolution) : String(item.resolution)}</p>
              </div>
            ) : null}

            <div className="border-t border-border pt-3">
              <p className="text-label text-muted-foreground">{view.labels.evidence}</p>
              {view.previewError ? <p role="alert" className="mt-1 text-label text-destructive">{view.previewError}</p> : null}
              {view.evidenceRows.length === 0 ? <p className="mt-1 text-label text-muted-foreground">{view.labels.noEvidence}</p> : null}
              <ul className="mt-1 divide-y divide-border">
                {view.evidenceRows.map((evidence) => (
                  <li key={evidence.id}>
                    <button type="button" onClick={() => void view.openEvidence(evidence)} className="flex w-full items-center gap-2 py-2 text-left text-body transition-colors duration-200 ease-in-out hover:bg-muted/40">
                      <FileText className="size-4 shrink-0 text-muted-foreground" aria-hidden="true" />
                      <span className="min-w-0 flex-1 truncate">{evidence.fileName}</span>
                      <span className="text-label tabular-nums text-muted-foreground">{evidence.size}</span>
                    </button>
                  </li>
                ))}
              </ul>
            </div>

            {view.canAct ? (
              <div className="flex flex-col gap-2 border-t border-border pt-3 sm:flex-row">
                <Button size="sm" variant="primary" icon={<CheckCircle2 />} onClick={() => view.openAction('close')}>{view.labels.closeCase}</Button>
                <Button size="sm" variant="outline" icon={<TriangleAlert />} disabled={Boolean(item.escalatedAt)} onClick={() => view.openAction('escalate')}>{view.labels.escalateCase}</Button>
              </div>
            ) : null}
          </div>
        ) : null}
      </Drawer>

      <Dialog
        open={view.action !== null}
        onOpenChange={(open) => (open ? undefined : view.setAction(null))}
        title={view.action === 'close' ? view.labels.closeTitle : view.labels.escalateTitle}
        description={view.action === 'close' ? view.labels.closeBody : view.labels.escalateBody}
        footer={
          <div className="flex justify-end gap-2">
            <Button variant="outline" disabled={view.busy} onClick={() => view.setAction(null)}>{view.labels.dismiss}</Button>
            <Button variant={view.action === 'close' ? 'destructive' : 'primary'} loading={view.busy} disabled={!view.text.trim()} onClick={() => void view.submit()}>{view.action === 'close' ? view.labels.confirmClose : view.labels.confirmEscalate}</Button>
          </div>
        }
      >
        <div className="flex flex-col gap-2">
          {view.formError ? <p role="alert" className="text-label text-destructive">{view.formError}</p> : null}
          <Textarea rows={4} maxLength={2000} value={view.text} placeholder={view.action === 'close' ? view.labels.resolutionPlaceholder : view.labels.escalationPlaceholder} onChange={(event) => view.setText(event.target.value)} aria-label={view.action === 'close' ? view.labels.resolution : view.labels.escalationReason} />
        </div>
      </Dialog>

      <Dialog open={view.preview !== null} onOpenChange={(open) => (open ? undefined : view.setPreview(null))} title={view.labels.previewTitle} description={view.preview?.fileName} footer={<Button variant="outline" onClick={() => view.setPreview(null)}>{view.labels.close}</Button>}>
        {view.preview && isImage ? <img src={`data:${view.preview.mimeType};base64,${view.preview.contentBase64}`} alt={view.preview.fileName} className="max-h-[70dvh] w-full object-contain" /> : null}
        {view.preview && isPdf ? <iframe title={view.preview.fileName} src={`data:application/pdf;base64,${view.preview.contentBase64}`} className="h-[70dvh] w-full" /> : null}
        {view.preview && !isImage && !isPdf ? <p className="text-body text-muted-foreground">{view.labels.previewUnavailable}</p> : null}
      </Dialog>
    </>
  );
}
