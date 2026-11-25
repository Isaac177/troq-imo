import { useMemo, type KeyboardEvent } from 'react';
import type { TableColumn, TableSelection } from './tableTypes';

type Options<Row> = {
  rows: Row[];
  columns: TableColumn<Row>[];
  getRowId: (row: Row) => string;
  page: number;
  pageSize: number;
  hiddenColumns?: string[];
  selection?: TableSelection;
  hasRowAction: boolean;
};

export const usePagedTable = <Row,>({ rows, columns, getRowId, page, pageSize, hiddenColumns, selection, hasRowAction }: Options<Row>) => {
  const visibleColumns = useMemo(() => columns.filter((column) => !hiddenColumns?.includes(column.key)), [columns, hiddenColumns]);
  const ids = useMemo(() => rows.map(getRowId), [rows, getRowId]);
  const selectedOnPage = selection ? ids.filter((id) => selection.selected.has(id)).length : 0;
  const allState: boolean | 'indeterminate' = selectedOnPage === 0 ? false : selectedOnPage === ids.length ? true : 'indeterminate';
  const firstStickyLeft = selection ? 'left-[72px] md:left-[96px]' : 'left-10 md:left-14';
  const columnCount = visibleColumns.length + 1 + (selection ? 1 : 0) + (hasRowAction ? 1 : 0);
  return {
    visibleColumns,
    ids,
    allState,
    firstStickyLeft,
    columnCount,
    rowNumber: (index: number) => (page - 1) * pageSize + index + 1,
    toggleAll: () => selection?.onToggleAll(ids, allState !== true),
    rowKeyDown: (event: KeyboardEvent<HTMLElement>, open: () => void) => {
      if (event.target !== event.currentTarget) return;
      if (event.key === 'Enter' || event.key === ' ') {
        event.preventDefault();
        open();
      }
    },
  };
};
