import { Eye, FileText, Upload } from 'lucide-react';
import { Button, Skeleton } from '@mungini/ui-web';
import type { CaseEvidenceView } from '@/hooks/cases/useCaseEvidence';

type Props = {
  view: CaseEvidenceView;
};

export default function CaseEvidenceList({ view }: Props) {
  return (
    <div className="flex flex-col gap-2">
      <div className="flex items-center justify-between gap-2">
        <p className="text-label text-muted-foreground">{view.labels.evidence}</p>
        {view.canUpload ? (
          <>
            <input ref={view.fileInput} type="file" accept="application/pdf,image/png,image/jpeg" className="sr-only" tabIndex={-1} onChange={view.chooseFile} />
            <Button
              size="sm"
              variant="outline"
              icon={<Upload />}
              collapseLabel
              loading={view.uploading}
              aria-label={view.labels.upload}
              title={view.labels.upload}
              onClick={view.pickFile}
            >
              {view.labels.upload}
            </Button>
          </>
        ) : null}
      </div>
      {view.canUpload ? <p className="text-label text-muted-foreground">{view.labels.uploadHint}</p> : null}
      {view.error ? <p role="alert" className="text-label text-destructive">{view.error}</p> : null}
      {view.notice ? <p role="status" className="text-label text-muted-foreground">{view.notice}</p> : null}
      {view.previewError ? <p role="alert" className="text-label text-destructive">{view.previewError}</p> : null}
      {view.loading ? <div className="space-y-2"><Skeleton shape="line" /><Skeleton shape="line" /></div> : null}
      {!view.loading && view.evidence.length === 0 ? <p className="text-label text-muted-foreground">{view.labels.noEvidence}</p> : null}
      <ul className="divide-y divide-border">
        {view.evidence.map((item) => (
          <li key={item.id} className="flex items-center gap-2 py-2">
            <FileText className="size-4 shrink-0 text-muted-foreground" aria-hidden="true" />
            <div className="min-w-0 flex-1">
              <p className="truncate text-body text-foreground">{item.fileName}</p>
              <p className="truncate text-label text-muted-foreground">
                {view.sizeText(item.sizeBytes)} · <span className="tabular-nums">{view.dateText(item.createdAt)}</span>
              </p>
              <p className="truncate text-label text-muted-foreground" title={item.addedByUserId}>{view.addedBy(item.addedByUserId)}</p>
            </div>
            <Button size="sm" variant="outline" icon={<Eye />} collapseLabel aria-label={view.labels.preview} title={view.labels.preview} onClick={() => void view.openPreview(item)}>{view.labels.preview}</Button>
          </li>
        ))}
      </ul>
    </div>
  );
}
