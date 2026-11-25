import { ArrowDown, ArrowUp, ChevronsUpDown } from 'lucide-react';
import { cn } from '../lib/cn';
import { useUiCopy } from '../copy/useUiCopy';
import { ariaSortOf, nextSortState } from './sortState';
import type { SortState } from './tableTypes';

type SortableHeaderProps = {
  label: string;
  sortKey: string;
  sort: SortState;
  onSortChange: (sort: SortState) => void;
  align?: 'left' | 'right';
  className?: string;
};

export const SortableHeader = ({ label, sortKey, sort, onSortChange, align = 'left', className }: SortableHeaderProps) => {
  const { table } = useUiCopy();
  const active = sort.sortBy === sortKey;
  const Icon = !active ? ChevronsUpDown : sort.sortDir === 'asc' ? ArrowUp : ArrowDown;
  return (
    <th scope="col" aria-sort={ariaSortOf(sort, sortKey)} className={cn('h-9 px-2 text-body font-medium text-muted-foreground', align === 'right' ? 'text-right' : 'text-left', className)}>
      <button
        type="button"
        title={table.sortBy(label)}
        onClick={() => onSortChange(nextSortState(sort, sortKey))}
        className={cn(
          'inline-flex h-8 items-center gap-1 rounded-md px-1 transition-colors duration-200 ease-in-out hover:text-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring',
          align === 'right' && 'flex-row-reverse',
          active && 'text-foreground',
        )}
      >
        {label}
        <Icon className={cn('size-3.5 shrink-0', !active && 'opacity-50')} aria-hidden="true" />
      </button>
    </th>
  );
};
