import { useEffect, useState } from 'react';

type Theme = 'light' | 'dark';
const storageKey = 'mungini-theme';

function initialTheme(): Theme {
  const stored = localStorage.getItem(storageKey);
  if (stored === 'light' || stored === 'dark') return stored;
  return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
}

export function useThemeToggle(lightLabel: string, darkLabel: string) {
  const [theme, setTheme] = useState<Theme>(initialTheme);

  useEffect(() => {
    document.documentElement.classList.toggle('dark', theme === 'dark');
    localStorage.setItem(storageKey, theme);
  }, [theme]);

  useEffect(() => {
    const sync = (event: StorageEvent) => {
      if (event.key === storageKey && (event.newValue === 'light' || event.newValue === 'dark')) {
        setTheme(event.newValue);
      }
    };
    window.addEventListener('storage', sync);
    return () => window.removeEventListener('storage', sync);
  }, []);

  return { isDark: theme === 'dark', label: theme === 'dark' ? lightLabel : darkLabel, toggle: () => setTheme(value => value === 'dark' ? 'light' : 'dark') };
}
