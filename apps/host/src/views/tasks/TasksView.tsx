import { Plus } from 'lucide-react';
import { Button, EmptyState, PagedTable, Section, Select, Skeleton } from '@mungini/ui-web';
import TaskCreateDialog from '@/components/tasks/TaskCreateDialog';
import { useHostTasks } from '@/hooks/tasks/useHostTasks';

export default function TasksView() {
  const view = useHostTasks();

  return (
    <>
      <Section
        title={view.labels.title}
        description={view.labels.subtitle}
        actions={
          <div className="flex w-full flex-col gap-2 sm:w-auto sm:flex-row sm:items-center">
            {view.hosts.length > 1 ? (
              <div className="w-full sm:w-[160px]">
                <Select options={view.hosts} value={view.hostId} onValueChange={view.setHostId} ariaLabel={view.labels.host} />
              </div>
            ) : null}
            <div className="w-full sm:w-[220px]">
              <Select options={view.propertyOptions} value={view.propertyId} onValueChange={view.setPropertyId} placeholder={view.labels.chooseProperty} ariaLabel={view.labels.property} />
            </div>
            <div className="w-full sm:w-[160px]">
              <Select options={view.stateOptions} value={view.stateValue} onValueChange={view.setState} ariaLabel={view.labels.state} />
            </div>
            <div className="w-full sm:w-[180px]">
              <Select options={view.typeOptions} value={view.typeValue} onValueChange={view.setType} ariaLabel={view.labels.type} />
            </div>
            <Button
              variant="primary"
              size="sm"
              icon={<Plus />}
              collapseLabel
              aria-label={view.labels.newTask}
              title={view.labels.newTask}
              disabled={!view.canCreate}
              onClick={view.create.openCreate}
            >
              {view.labels.newTask}
            </Button>
          </div>
        }
      >
        {view.notice ? <p role="status" className="text-label text-muted-foreground">{view.notice}</p> : null}
        {view.actionError ? <p role="alert" className="text-label text-destructive">{view.actionError}</p> : null}
        {!view.hasHost ? (
          <EmptyState title={view.labels.noHost} />
        ) : view.forbidden ? (
          <EmptyState title={view.labels.forbidden} />
        ) : view.propertiesLoading ? (
          <div className="flex flex-col gap-2">
            <Skeleton shape="row" />
            <Skeleton shape="row" />
            <Skeleton shape="row" />
          </div>
        ) : view.error ? (
          <div role="alert">
            <EmptyState title={view.error} />
          </div>
        ) : !view.propertyId ? (
          <EmptyState title={view.labels.noProperties} />
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
            emptyState={<EmptyState title={view.labels.empty} />}
          />
        )}
      </Section>
      <TaskCreateDialog view={view.create} />
    </>
  );
}
