import { Download } from 'lucide-react';
import { Button, Dialog } from '@mungini/ui-web';
import type { CaseEvidenceView } from '@/hooks/cases/useCaseEvidence';

type Props = {
  view: CaseEvidenceView;
};

export default function CaseEvidencePreview({ view }: Props) {
  const content = view.preview;
  return (
    <Dialog
      open={content !== null}
      onOpenChange={(open) => (open ? undefined : view.closePreview())}
      title={view.labels.previewTitle}
      description={content?.evidence.fileName}
      footer={
        <div className="flex justify-end gap-2">
          <Button variant="outline" size="sm" onClick={view.closePreview}>{view.labels.closePreview}</Button>
          {content ? <Button variant="primary" size="sm" icon={<Download />} onClick={view.downloadPreview}>{view.labels.download}</Button> : null}
        </div>
      }
    >
      {content && view.previewKind === 'image' ? (
        <img src={`data:${content.evidence.mimeType};base64,${content.contentBase64}`} alt={content.evidence.fileName} className="max-h-[70dvh] w-full object-contain" />
      ) : null}
      {content && view.previewKind === 'pdf' ? (
        <iframe title={content.evidence.fileName} src={`data:application/pdf;base64,${content.contentBase64}`} className="h-[70dvh] w-full" />
      ) : null}
      {content && view.previewKind === 'other' ? <p className="text-body text-muted-foreground">{view.labels.previewUnavailable}</p> : null}
    </Dialog>
  );
}
