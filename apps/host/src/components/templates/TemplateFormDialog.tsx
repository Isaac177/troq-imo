import { Button, Dialog } from '@mungini/ui-web';
import TemplateFormFields from './TemplateFormFields';
import TemplateMediaSection from './TemplateMediaSection';
import type { TemplateEditorView } from '@/hooks/templates/useTemplateEditor';

type Props = { view: TemplateEditorView; onChanged: () => void };

export default function TemplateFormDialog({ view, onChanged }: Props) {
  return (
    <Dialog
      open={view.open}
      onOpenChange={view.setOpen}
      title={view.target ? view.labels.editTitle : view.labels.createTitle}
      description={view.labels.formHint}
      size="lg"
      footer={
        <div className="flex justify-end gap-2">
          <Button variant="outline" disabled={view.busy} onClick={() => view.setOpen(false)}>{view.labels.cancel}</Button>
          <Button variant="primary" loading={view.busy} disabled={!view.canSubmit} onClick={() => void view.submit()}>{view.labels.save}</Button>
        </div>
      }
    >
      <div className="flex flex-col gap-4">
        {view.formError ? <p role="alert" className="text-label text-destructive">{view.formError}</p> : null}
        <TemplateFormFields view={view} />
        {view.target ? <TemplateMediaSection templateId={view.target.id} onChanged={onChanged} /> : null}
      </div>
    </Dialog>
  );
}
