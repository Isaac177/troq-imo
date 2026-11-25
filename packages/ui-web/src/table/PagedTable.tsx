import type { ReactNode } from 'react';
import { cn } from '../lib/cn';
import { Checkbox } from '../form/Checkbox';
import { Skeleton } from '../display/Skeleton';
import { useUiCopy } from '../copy/useUiCopy';
import { RowContextMenu } from '../menu/RowContextMenu';
import type { ActionItem } from '../menu/actionItem';
import { Pagination } from './Pagination';
import { PagedTableRow } from './PagedTableRow';
import { SortableHeader } from './SortableHeader';
import { hideBelowClasses, numberColumnClasses, selectColumnClasses, stickyHeadClasses } from './tableClasses';
import type { SortState, TableColumn, TableSelection } from './tableTypes';
import { usePagedTable } from './usePagedTable';

type PagedTableProps<Row> = {
  rows: Row[];
  columns: TableColumn<Row>[];
  getRowId: (row: Row) => string;
  page: number;
  pageSize: number;
  total: number;
  onPageChange: (page: number) => void;
  ariaLabel: string;
  sort?: SortState;
  onSortChange?: (sort: SortState) => void;
  loading?: boolean;
  error?: string | null;
  emptyState?: ReactNode;
  onRowClick?: (row: Row) => void;
  rowActions?: (row: Row) => ActionItem[];
  selection?: TableSelection;
  hiddenColumns?: string[];
  footerSlot?: ReactNode;
  className?: string;
};

const SKELETON_ROWS = 5;

export const PagedTable = <Row,>({
  rows,
  columns,
  getRowId,
  page,
  pageSize,
  total,
  onPageChange,
  ariaLabel,
  sort,
  onSortChange,
  loading = false,
  error,
  emptyState,
  onRowClick,
  rowActions,
  selection,
  hiddenColumns,
  footerSlot,
  className,
}: PagedTableProps<Row>) => {
  const { table } = useUiCopy();
  const view = usePagedTable({ rows, columns, getRowId, page, pageSize, hiddenColumns, selection, hasRowAction: onRowClick !== undefined });
  const firstLoad = loading && rows.length === 0;
  const showEmpty = !loading && !error && rows.length === 0;
  return (
    <div className={cn('flex min-w-0 flex-col', className)}>
      <div className={cn('min-w-0 overflow-x-auto transition-opacity duration-200 ease-in-out', loading && !firstLoad && 'opacity-60')} aria-busy={loading}>
        <table aria-label={ariaLabel} className="w-full border-collapse text-body">
          <thead className="border-b border-border bg-muted/40">
            <tr>
              <th scope="col" className={cn('h-9 text-body font-medium text-muted-foreground', numberColumnClasses, stickyHeadClasses, 'left-0')}>
                #
              </th>
              {selection ? (
                <th scope="col" className={cn(selectColumnClasses, stickyHeadClasses, 'left-10 md:left-14')}>
                  <Checkbox checked={view.allState} onCheckedChange={view.toggleAll} ariaLabel={table.selectAll} />
                </th>
              ) : null}
              {view.visibleColumns.map((column) => {
                const stickyClasses = column.sticky ? cn(stickyHeadClasses, view.firstStickyLeft) : undefined;
                const visibility = column.hideBelow ? hideBelowClasses[column.hideBelow] : undefined;
                if (column.sortable && sort && onSortChange) {
                  return <SortableHeader key={column.key} label={column.header} sortKey={column.key} sort={sort} onSortChange={onSortChange} align={column.align} className={cn(visibility, stickyClasses)} />;
                }
                return (
                  <th key={column.key} scope="col" className={cn('h-9 px-2 text-body font-medium text-muted-foreground', column.align === 'right' ? 'text-right' : 'text-left', visibility, stickyClasses)}>
                    {column.headerHidden ? <span className="sr-only">{column.header}</span> : column.header}
                  </th>
                );
              })}
              {onRowClick ? (
                <th scope="col" className="w-8 pr-3">
                  <span className="sr-only">{table.openRecord}</span>
                </th>
              ) : null}
            </tr>
          </thead>
          <tbody>
            {firstLoad
              ? Array.from({ length: SKELETON_ROWS }, (_, index) => (
                  <tr key={index} className="border-b border-border last:border-b-0">
                    <td colSpan={view.columnCount} className="px-3 py-2">
                      <Skeleton shape="row" />
                    </td>
                  </tr>
                ))
              : rows.map((row, index) => {
                  const id = view.ids[index] ?? String(index);
                  const open = onRowClick ? () => onRowClick(row) : undefined;
                  const line = (
                    <PagedTableRow
                      key={id}
                      row={row}
                      id={id}
                      rowNumber={view.rowNumber(index)}
                      columns={view.visibleColumns}
                      selection={selection}
                      onOpen={open}
                      onKeyDown={open ? (event) => view.rowKeyDown(event, open) : undefined}
                      firstStickyLeft={view.firstStickyLeft}
                    />
                  );
                  return rowActions ? (
                    <RowContextMenu key={id} items={rowActions(row)}>
                      {line}
                    </RowContextMenu>
                  ) : (
                    line
                  );
                })}
          </tbody>
        </table>
      </div>
      {error ? <div role="alert" className="px-4 py-10 text-center text-body text-tone-problem">{error}</div> : null}
      {showEmpty ? <div className="animate-fade-in">{emptyState}</div> : null}
      {footerSlot}
      <Pagination page={page} pageSize={pageSize} total={total} onPageChange={onPageChange} />
    </div>
  );
};
