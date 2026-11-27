import { Button, Dialog, Select, Textarea } from '@mungini/ui-web';
import type { TaskCreateView } from '@/hooks/tasks/useTaskCreate';
import type { TaskPriority } from '@/types/tasks/tasks';

type Props = {
  view: TaskCreateView;
};

export default function TaskCreateDialog({ view }: Props) {
  return (
    <Dialog
      open={view.open}
      onOpenChange={view.onOpenChange}
      title={view.labels.title}
      description={view.labels.description}
      footer={
        <div className="flex justify-end gap-2">
          <Button variant="outline" size="sm" disabled={view.busy} onClick={view.close}>{view.labels.cancel}</Button>
          <Button variant="primary" size="sm" loading={view.busy} disabled={!view.canSubmit} onClick={() => void view.submit()}>{view.labels.submit}</Button>
        </div>
      }
    >
      <div className="flex flex-col gap-3">
        {view.formError ? <p role="alert" className="text-label text-destructive">{view.formError}</p> : null}
        <div className="flex flex-col gap-1 text-label text-muted-foreground">
          <span>{view.labels.property}</span>
          <p className="text-body font-medium text-foreground">{view.propertyName || view.labels.chooseProperty}</p>
        </div>
        <label className="flex flex-col gap-1 text-label text-muted-foreground">
          {view.labels.type}
          <Select options={view.typeOptions} value={view.taskType} onValueChange={view.setTaskType} ariaLabel={view.labels.type} />
        </label>
        <label className="flex flex-col gap-1 text-label text-muted-foreground">
          {view.labels.priority}
          <Select options={view.priorityOptions} value={view.priority} onValueChange={(value) => view.setPriority(value as TaskPriority)} ariaLabel={view.labels.priority} />
        </label>
        <label className="flex flex-col gap-1 text-label text-muted-foreground">
          {view.labels.checklist}
          <Textarea rows={4} value={view.checklist} placeholder={view.labels.checklistPlaceholder} onChange={(event) => view.setChecklist(event.target.value)} aria-label={view.labels.checklist} />
          <span>{view.labels.checklistHint}</span>
        </label>
      </div>
    </Dialog>
  );
}
