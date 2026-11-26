import { Search } from 'lucide-react';
import { EmptyState, Input, PagedTable, Section, Select } from '@mungini/ui-web';
import AdminPropertyDrawer from '@/components/properties/AdminPropertyDrawer';
import { useAdminProperties } from '@/hooks/properties/useAdminProperties';

export default function PropertiesView() {
  const view = useAdminProperties();
  return (
    <>
      <Section
        title={view.labels.title}
        description={view.labels.subtitle}
        actions={
          <div className="flex w-full flex-col gap-2 sm:w-auto sm:flex-row">
            <div className="w-full sm:w-[180px]">
              <Select options={view.statusOptions} value={view.status} onValueChange={view.setStatus} ariaLabel={view.labels.status} />
            </div>
            <div className="w-full sm:w-[260px]">
              <Input type="search" value={view.search} onChange={(event) => view.setSearch(event.target.value)} placeholder={view.labels.search} aria-label={view.labels.search} leadingIcon={<Search />} />
            </div>
          </div>
        }
      >
        {view.error ? <p role="alert" className="text-label text-destructive">{view.error}</p> : null}
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
          onRowClick={(row) => view.setSelected(row.id)}
          emptyState={<EmptyState title={view.labels.empty} />}
        />
      </Section>
      <AdminPropertyDrawer propertyId={view.selected} onClose={() => view.setSelected(null)} onChanged={view.onChanged} />
    </>
  );
}
