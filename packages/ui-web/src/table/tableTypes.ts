import type { ReactNode } from 'react';

export type SortDirection = 'asc' | 'desc';

export type SortState = { sortBy: string | null; sortDir: SortDirection };

export type TableColumn<Row> = {
  key: string;
  header: string;
  cell: (row: Row, rowNumber: number) => ReactNode;
  hideBelow?: 'sm' | 'md' | 'lg';
  align?: 'left' | 'right';
  sortable?: boolean;
  headerHidden?: boolean;
  sticky?: boolean;
  width?: number;
  className?: string;
};

export type TableSelection = {
  selected: ReadonlySet<string>;
  onToggle: (id: string) => void;
  onToggleAll: (ids: string[], checked: boolean) => void;
};
