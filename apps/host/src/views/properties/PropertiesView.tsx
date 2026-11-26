import { Plus, Search } from 'lucide-react';
import { Button, EmptyState, Input, PagedTable, Section, Select } from '@mungini/ui-web';
import PropertyCreateDialog from '@/components/properties/PropertyCreateDialog';
import PropertyDrawer from '@/components/properties/PropertyDrawer';
import { useProperties } from '@/hooks/properties/useProperties';

export default function PropertiesView() {
  const view = useProperties();

  return (
    <>
      <Section
        title={view.labels.title}
        description={view.labels.subtitle}
        actions={
          <div className="flex w-full flex-col gap-2 sm:w-auto sm:flex-row">
            {view.hosts.length > 1 ? (
              <div className="w-full sm:w-[180px]">
                <Select
                  options={view.hosts.map((host) => ({ value: host.hostId, label: host.displayName }))}
                  value={view.hostId}
                  onValueChange={view.setHostId}
                  ariaLabel={view.labels.title}
                />
              </div>
            ) : null}
            <div className="w-full sm:w-[220px]">
              <Input
                type="search"
                value={view.search}
                onChange={(event) => view.setSearch(event.target.value)}
                placeholder={view.labels.search}
                aria-label={view.labels.search}
                leadingIcon={<Search />}
              />
            </div>
            <Button variant="primary" size="sm" icon={<Plus />} collapseLabel aria-label={view.labels.newProperty} title={view.labels.newProperty} disabled={!view.timezone} onClick={view.openCreate}>{view.labels.newProperty}</Button>
          </div>
        }
      >
        {view.error ? <p role="alert" className="text-label text-destructive">{view.error}</p> : null}
        {view.notice ? <p role="status" className="text-label text-muted-foreground">{view.notice}</p> : null}
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
          onRowClick={(row) => view.setSelected(row)}
          emptyState={<EmptyState title={view.labels.empty} />}
        />
      </Section>
      <PropertyCreateDialog view={view} />
      <PropertyDrawer view={view} />
    </>
  );
}
