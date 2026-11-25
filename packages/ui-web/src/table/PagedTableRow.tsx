import type { ComponentProps, KeyboardEvent } from 'react';
import { ChevronRight } from 'lucide-react';
import { cn } from '../lib/cn';
import { Checkbox } from '../form/Checkbox';
import { useUiCopy } from '../copy/useUiCopy';
import { hideBelowClasses, numberColumnClasses, selectColumnClasses, stickyCellClasses } from './tableClasses';
import type { TableColumn, TableSelection } from './tableTypes';

type PagedTableRowProps<Row> = Omit<ComponentProps<'tr'>, 'children' | 'onClick' | 'onKeyDown' | 'id'> & {
  row: Row;
  id: string;
  rowNumber: number;
  columns: TableColumn<Row>[];
  selection?: TableSelection;
  onOpen?: () => void;
  onKeyDown?: (event: KeyboardEvent<HTMLTableRowElement>) => void;
  firstStickyLeft: string;
};

export const PagedTableRow = <Row,>({ row, id, rowNumber, columns, selection, onOpen, onKeyDown, firstStickyLeft, className, ...rest }: PagedTableRowProps<Row>) => {
  const { table } = useUiCopy();
  const selected = selection?.selected.has(id) ?? false;
  return (
    <tr
      {...rest}
      data-selected={selected}
      tabIndex={onOpen ? 0 : undefined}
      onClick={onOpen}
      onKeyDown={onKeyDown}
      className={cn(
        'group/row border-b border-border transition-colors duration-200 ease-in-out last:border-b-0 hover:bg-muted/50 data-[selected=true]:bg-accent focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-inset focus-visible:ring-ring',
        onOpen && 'cursor-pointer',
        className,
      )}
    >
      <td className={cn(numberColumnClasses, stickyCellClasses, 'left-0')}>{rowNumber}</td>
      {selection ? (
        <td className={cn(selectColumnClasses, stickyCellClasses, 'left-10 md:left-14')} onClick={(event) => event.stopPropagation()}>
          <Checkbox checked={selected} onCheckedChange={() => selection.onToggle(id)} ariaLabel={table.selectRow(rowNumber)} />
        </td>
      ) : null}
      {columns.map((column) => (
        <td
          key={column.key}
          className={cn(
            'px-2 py-2.5 align-middle text-body text-foreground',
            column.align === 'right' && 'text-right tabular-nums',
            column.hideBelow && hideBelowClasses[column.hideBelow],
            column.sticky && cn(stickyCellClasses, firstStickyLeft),
            column.className,
          )}
        >
          {column.cell(row, rowNumber)}
        </td>
      ))}
      {onOpen ? (
        <td className="w-8 pr-3 text-right text-muted-foreground">
          <ChevronRight className="ml-auto size-4" aria-hidden="true" />
        </td>
      ) : null}
    </tr>
  );
};
