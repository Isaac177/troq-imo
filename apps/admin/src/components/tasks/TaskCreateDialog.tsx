import { Plus } from 'lucide-react';
import { Button, Dialog, Select, Textarea } from '@mungini/ui-web';
import type { AdminTasksView } from '@/hooks/tasks/useAdminTasks';

export default function TaskCreateDialog({ create }: { create: AdminTasksView['create'] }) {
  return (
    <Dialog
      open={create.open}
      onOpenChange={(open) => (open ? create.openDialog() : create.closeDialog())}
      title={create.labels.title}
      description={create.labels.description}
      footer={
        <div className="flex justify-end gap-2">
          <Button variant="outline" disabled={create.busy} onClick={create.closeDialog}>{create.labels.cancel}</Button>
          <Button icon={<Plus />} loading={create.busy} disabled={!create.propertyId} onClick={create.submit}>{create.labels.submit}</Button>
        </div>
      }
    >
      <div className="flex flex-col gap-3">
        {create.error ? <p role="alert" className="text-label text-destructive">{create.error}</p> : null}
        <label className="flex flex-col gap-1 text-label text-muted-foreground">
          {create.labels.host}
          <Select options={create.hostOptions} value={create.hostId} onValueChange={create.setHostId} ariaLabel={create.labels.host} />
        </label>
        <label className="flex flex-col gap-1 text-label text-muted-foreground">
          {create.labels.property}
          <Select options={create.propertyOptions} value={create.propertyId} onValueChange={create.setPropertyId} placeholder={create.labels.propertyPlaceholder} ariaLabel={create.labels.property} invalid={!create.propertyId && Boolean(create.error)} />
        </label>
        <div className="grid grid-cols-1 gap-3 sm:grid-cols-2">
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {create.labels.type}
            <Select options={create.typeOptions} value={create.taskType} onValueChange={create.setTaskType} ariaLabel={create.labels.type} />
          </label>
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {create.labels.priority}
            <Select options={create.priorityOptions} value={create.priority} onValueChange={create.setPriority} ariaLabel={create.labels.priority} />
          </label>
        </div>
        <label className="flex flex-col gap-1 text-label text-muted-foreground">
          {create.labels.checklist}
          <Textarea rows={5} value={create.checklistText} onChange={(event) => create.setChecklistText(event.target.value)} placeholder={create.labels.checklistPlaceholder} aria-label={create.labels.checklist} />
        </label>
      </div>
    </Dialog>
  );
}
