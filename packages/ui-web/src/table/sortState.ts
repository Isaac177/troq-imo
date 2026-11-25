import type { SortState } from './tableTypes';

export const nextSortState = (current: SortState, key: string): SortState => {
  if (current.sortBy !== key) return { sortBy: key, sortDir: 'asc' };
  return { sortBy: key, sortDir: current.sortDir === 'asc' ? 'desc' : 'asc' };
};

export const ariaSortOf = (current: SortState, key: string): 'ascending' | 'descending' | 'none' => {
  if (current.sortBy !== key) return 'none';
  return current.sortDir === 'asc' ? 'ascending' : 'descending';
};
