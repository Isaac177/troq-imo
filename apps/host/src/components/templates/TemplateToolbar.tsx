import { Search, X } from 'lucide-react';
import { Button, Input, Select } from '@mungini/ui-web';
import type { HostTemplatesView } from '@/hooks/templates/useHostTemplates';

type Props = { view: HostTemplatesView };

export default function TemplateToolbar({ view }: Props) {
  return (
    <div className="flex flex-col gap-2 md:flex-row md:items-center md:justify-between">
      <form className="flex items-center gap-2 md:w-80" onSubmit={view.onSearchSubmit}>
        <Input
          type="search"
          value={view.searchInput}
          onChange={(event) => view.setSearchInput(event.target.value)}
          placeholder={view.labels.searchPlaceholder}
          aria-label={view.labels.searchLabel}
          leadingIcon={<Search />}
        />
        <Button type="submit" variant="outline" size="sm" icon={<Search />} collapseLabel aria-label={view.labels.searchAction} title={view.labels.searchAction}>{view.labels.searchAction}</Button>
        {view.hasFilters ? <Button type="button" variant="ghost" size="icon" aria-label={view.labels.clearFilters} title={view.labels.clearFilters} onClick={view.clearFilters}><X /></Button> : null}
      </form>
      <div className="flex flex-col gap-2 sm:flex-row">
        <div className="w-full sm:w-[180px]">
          <Select options={view.statusOptions} value={view.status} onValueChange={view.setStatus} ariaLabel={view.labels.status} />
        </div>
        <div className="w-full sm:w-[200px]">
          <Select options={view.categoryOptions} value={view.category} onValueChange={view.setCategory} ariaLabel={view.labels.category} />
        </div>
      </div>
    </div>
  );
}
