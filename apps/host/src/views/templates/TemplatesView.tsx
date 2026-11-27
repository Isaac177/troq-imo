import { Link } from 'react-router-dom';
import { Plus } from 'lucide-react';
import { Button, EmptyState, Section, Select } from '@mungini/ui-web';
import TemplateFormDialog from '@/components/templates/TemplateFormDialog';
import TemplateList from '@/components/templates/TemplateList';
import TemplateToolbar from '@/components/templates/TemplateToolbar';
import { useHostTemplates } from '@/hooks/templates/useHostTemplates';
import { useTemplateEditor } from '@/hooks/templates/useTemplateEditor';

export default function TemplatesView() {
  const view = useHostTemplates();
  const editor = useTemplateEditor({ hostId: view.hostId, categories: view.categories, onSaved: view.afterSave });

  if (!view.hasHost) {
    return (
      <Section title={view.labels.title} description={view.labels.subtitle}>
        <EmptyState title={view.labels.noHost} action={<Button asChild variant="primary" size="sm"><Link to="/onboarding">{view.labels.noHostAction}</Link></Button>} />
      </Section>
    );
  }

  return (
    <>
      <Section
        title={view.labels.title}
        description={view.labels.subtitle}
        actions={
          <div className="flex w-full flex-col gap-2 sm:w-auto sm:flex-row">
            {view.hosts.length > 1 ? (
              <div className="w-full sm:w-[180px]">
                <Select options={view.hosts.map((host) => ({ value: host.hostId, label: host.displayName }))} value={view.hostId} onValueChange={view.setHostId} ariaLabel={view.labels.title} />
              </div>
            ) : null}
            {view.canWrite ? (
              <Button variant="primary" size="sm" icon={<Plus />} collapseLabel aria-label={view.labels.newTemplate} title={view.labels.newTemplate} onClick={editor.openCreate}>{view.labels.newTemplate}</Button>
            ) : null}
          </div>
        }
      >
        <TemplateToolbar view={view} />
        {view.forbidden ? (
          <EmptyState title={view.labels.forbidden} />
        ) : (
          <>
            {view.error ? <p role="alert" className="text-label text-destructive">{view.error}</p> : null}
            {view.notice ? <p role="status" className="text-label text-muted-foreground">{view.notice}</p> : null}
            <TemplateList view={view} onEdit={editor.openEdit} />
          </>
        )}
      </Section>
      <TemplateFormDialog view={editor} onChanged={view.reload} />
    </>
  );
}
