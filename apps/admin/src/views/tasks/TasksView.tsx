import { Plus, Search, ShieldAlert } from 'lucide-react';
import { Button, EmptyState, Input, PagedTable, Section, Select } from '@mungini/ui-web';
import TaskCreateDialog from '@/components/tasks/TaskCreateDialog';
import { useAdminTasks } from '@/hooks/tasks/useAdminTasks';

export default function TasksView() {
  const view = useAdminTasks();
  return (
    <>
      <Section
        title={view.labels.title}
        description={view.labels.subtitle}
        actions={
          <Button icon={<Plus />} collapseLabel aria-label={view.labels.newTask} title={view.labels.newTask} onClick={view.openCreate}>
            {view.labels.newTask}
          </Button>
        }
      >
        <div className="grid grid-cols-1 gap-2 sm:grid-cols-2 lg:grid-cols-4">
          <Select options={view.hostOptions} value={view.hostId} onValueChange={view.changeHost} ariaLabel={view.labels.host} />
          <Select options={view.propertyOptions} value={view.propertyId} onValueChange={view.setPropertyId} ariaLabel={view.labels.property} />
          <Select options={view.stateOptions} value={view.state} onValueChange={view.setState} ariaLabel={view.labels.state} />
          <Select options={view.typeOptions} value={view.taskType} onValueChange={view.setTaskType} ariaLabel={view.labels.taskType} />
          <Select options={view.priorityOptions} value={view.priority} onValueChange={view.setPriority} ariaLabel={view.labels.priority} />
          <Select options={view.assigneeOptions} value={view.assignee} onValueChange={view.setAssignee} ariaLabel={view.labels.assignee} />
          <div className="sm:col-span-2">
            <Input type="search" value={view.search} onChange={(event) => view.setSearch(event.target.value)} placeholder={view.labels.search} aria-label={view.labels.search} leadingIcon={<Search />} />
          </div>
        </div>
        {view.actionError ? <p role="alert" className="text-label text-destructive">{view.actionError}</p> : null}
        {view.notice ? <p role="status" className="text-label text-muted-foreground">{view.notice}</p> : null}
        {view.forbidden ? (
          <EmptyState title={view.labels.forbidden} description={view.labels.forbiddenHint} icon={<ShieldAlert />} />
        ) : (
          <PagedTable
            rows={view.rows}
            columns={view.columns}
            getRowId={(row) => row.id}
            page={view.page}
            pageSize={view.pageSize}
            total={view.total}
            onPageChange={view.setPage}
            ariaLabel={view.labels.title}
            loading={view.loading}
            error={view.error}
            emptyState={<EmptyState title={view.labels.empty} description={view.labels.emptyHint} />}
          />
        )}
      </Section>
      <TaskCreateDialog create={view.create} />
    </>
  );
}
