import { EmptyState, PagedTable } from '@mungini/ui-web';
import type { HostTemplatesView } from '@/hooks/templates/useHostTemplates';
import type { Template } from '@/types/templates/templates';

type Props = { view: HostTemplatesView; onEdit: (template: Template) => void };

export default function TemplateList({ view, onEdit }: Props) {
  return (
    <PagedTable
      rows={view.items}
      columns={view.columns}
      getRowId={(row) => row.id}
      page={view.page}
      pageSize={view.pageSize}
      total={view.total}
      onPageChange={view.setPage}
      ariaLabel={view.labels.title}
      loading={view.loading}
      onRowClick={view.canWrite ? onEdit : undefined}
      emptyState={<EmptyState title={view.labels.empty} description={view.labels.emptyHint} />}
    />
  );
}
