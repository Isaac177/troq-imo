import { useCallback, useEffect, useMemo, useState } from 'react';

export type GridColumnDef = {
  key: string;
  defaultWidth?: number;
  hideable?: boolean;
  defaultHidden?: boolean;
};

type StoredPrefs = { widths: Record<string, number>; hidden: string[] };

const STORAGE_VERSION = 'v1';

const storageKeyOf = (key: string): string => `mungini:grid:${STORAGE_VERSION}:${key}`;

const readPrefs = (key: string): StoredPrefs | null => {
  try {
    const raw = window.localStorage.getItem(storageKeyOf(key));
    if (!raw) return null;
    const parsed: unknown = JSON.parse(raw);
    if (typeof parsed !== 'object' || parsed === null) return null;
    const { widths, hidden } = parsed as Partial<StoredPrefs>;
    return { widths: widths && typeof widths === 'object' ? widths : {}, hidden: Array.isArray(hidden) ? hidden.filter((item) => typeof item === 'string') : [] };
  } catch {
    return null;
  }
};

const writePrefs = (key: string, prefs: StoredPrefs): void => {
  try {
    window.localStorage.setItem(storageKeyOf(key), JSON.stringify(prefs));
  } catch {
    return;
  }
};

export const useGridPrefs = (storageKey: string, columns: GridColumnDef[]) => {
  const columnsKey = JSON.stringify(columns.map(({ key, defaultWidth, hideable, defaultHidden }) => ({ key, defaultWidth, hideable, defaultHidden })));
  const stableColumns = useMemo<GridColumnDef[]>(() => JSON.parse(columnsKey), [columnsKey]);
  const defaults = useMemo<StoredPrefs>(
    () => ({
      widths: Object.fromEntries(stableColumns.filter((column) => column.defaultWidth !== undefined).map((column) => [column.key, column.defaultWidth as number])),
      hidden: stableColumns.filter((column) => column.defaultHidden && column.hideable !== false).map((column) => column.key),
    }),
    [stableColumns],
  );
  const allowedKeys = useMemo(() => new Set(stableColumns.map((column) => column.key)), [stableColumns]);
  const hideableKeys = useMemo(() => new Set(stableColumns.filter((column) => column.hideable !== false).map((column) => column.key)), [stableColumns]);
  const load = useCallback(() => {
    const stored = readPrefs(storageKey);
    if (!stored) return defaults;
    return {
      widths: { ...defaults.widths, ...Object.fromEntries(Object.entries(stored.widths).filter(([key, width]) => allowedKeys.has(key) && typeof width === 'number' && Number.isFinite(width) && width >= 48)) },
      hidden: stored.hidden.filter((key) => hideableKeys.has(key)),
    };
  }, [storageKey, defaults, allowedKeys, hideableKeys]);
  const [prefs, setPrefs] = useState<StoredPrefs>(load);
  const [selected, setSelected] = useState<ReadonlySet<string>>(() => new Set());

  useEffect(() => {
    setPrefs(load());
    setSelected(new Set());
  }, [load]);

  const update = useCallback(
    (change: (current: StoredPrefs) => StoredPrefs) => {
      setPrefs((current) => {
        const next = change(current);
        writePrefs(storageKey, next);
        return next;
      });
    },
    [storageKey],
  );

  const setWidth = useCallback((key: string, width: number) => {
    if (!allowedKeys.has(key) || !Number.isFinite(width)) return;
    update((current) => ({ ...current, widths: { ...current.widths, [key]: Math.max(48, Math.round(width)) } }));
  }, [update, allowedKeys]);

  const toggleColumn = useCallback(
    (key: string) => {
      if (!hideableKeys.has(key)) return;
      update((current) => ({
        ...current,
        hidden: current.hidden.includes(key) ? current.hidden.filter((item) => item !== key) : [...current.hidden, key],
      }));
    },
    [update, hideableKeys],
  );

  const resetPrefs = useCallback(() => update(() => defaults), [update, defaults]);

  const toggleRow = useCallback(
    (id: string) =>
      setSelected((current) => {
        const next = new Set(current);
        if (next.has(id)) next.delete(id);
        else next.add(id);
        return next;
      }),
    [],
  );

  const toggleAll = useCallback(
    (ids: string[], checked: boolean) =>
      setSelected((current) => {
        const next = new Set(current);
        ids.forEach((id) => (checked ? next.add(id) : next.delete(id)));
        return next;
      }),
    [],
  );

  const clearSelection = useCallback(() => setSelected(new Set()), []);

  return {
    widths: prefs.widths,
    hiddenColumns: prefs.hidden,
    setWidth,
    toggleColumn,
    resetPrefs,
    selection: { selected, onToggle: toggleRow, onToggleAll: toggleAll },
    selectedCount: selected.size,
    clearSelection,
  };
};
