import { Button, Dialog, Textarea } from '@mungini/ui-web';
import type { CaseDetailView } from '@/hooks/cases/useCaseDetail';

type Props = {
  view: CaseDetailView;
};

export default function CaseCloseDialog({ view }: Props) {
  return (
    <Dialog
      open={view.closeOpen}
      onOpenChange={(open) => (open ? undefined : view.dismissClose())}
      title={view.labels.closeTitle}
      description={view.labels.closeBody}
      footer={
        <div className="flex justify-end gap-2">
          <Button variant="outline" size="sm" disabled={view.busy} onClick={view.dismissClose}>{view.labels.dismiss}</Button>
          <Button variant="primary" size="sm" loading={view.busy} disabled={!view.canSubmitClose} onClick={() => void view.submitClose()}>{view.labels.confirmClose}</Button>
        </div>
      }
    >
      <div className="flex flex-col gap-2">
        {view.formError ? <p role="alert" className="text-label text-destructive">{view.formError}</p> : null}
        <Textarea
          rows={4}
          maxLength={2000}
          value={view.resolution}
          placeholder={view.labels.resolutionPlaceholder}
          onChange={(event) => view.setResolution(event.target.value)}
          aria-label={view.labels.resolutionLabel}
        />
      </div>
    </Dialog>
  );
}
