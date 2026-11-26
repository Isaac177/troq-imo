import { useCallback, useEffect, useRef, useState } from 'react';

type Theme = 'light' | 'dark';
const storageKey = 'mungini-theme';

function readStoredTheme(): Theme | null {
  try {
    const stored = window.localStorage.getItem(storageKey);
    return stored === 'light' || stored === 'dark' ? stored : null;
  } catch {
    return null;
  }
}

function storeTheme(theme: Theme): void {
  try {
    window.localStorage.setItem(storageKey, theme);
  } catch {
    return;
  }
}

export function useThemeToggle(lightLabel: string, darkLabel: string) {
  const [theme, setTheme] = useState<Theme>('light');
  const currentTheme = useRef<Theme>('light');
  const explicit = useRef(false);

  const applyTheme = useCallback((next: Theme) => {
    currentTheme.current = next;
    document.documentElement.classList.toggle('dark', next === 'dark');
    setTheme(next);
  }, []);

  useEffect(() => {
    const media = window.matchMedia?.('(prefers-color-scheme: dark)');
    const systemTheme = (): Theme => media?.matches ? 'dark' : 'light';
    const stored = readStoredTheme();
    explicit.current = stored !== null;
    applyTheme(stored ?? systemTheme());

    const onSystemChange = () => {
      if (!explicit.current) applyTheme(systemTheme());
    };
    const onStorage = (event: StorageEvent) => {
      if (event.key !== storageKey && event.key !== null) return;
      const value = event.key === null ? null : event.newValue;
      explicit.current = value === 'light' || value === 'dark';
      applyTheme(explicit.current ? value as Theme : systemTheme());
    };
    media?.addEventListener('change', onSystemChange);
    window.addEventListener('storage', onStorage);
    return () => {
      media?.removeEventListener('change', onSystemChange);
      window.removeEventListener('storage', onStorage);
    };
  }, [applyTheme]);

  const toggle = useCallback(() => {
    const next = currentTheme.current === 'dark' ? 'light' : 'dark';
    explicit.current = true;
    applyTheme(next);
    storeTheme(next);
  }, [applyTheme]);

  return { isDark: theme === 'dark', label: theme === 'dark' ? lightLabel : darkLabel, toggle };
}
