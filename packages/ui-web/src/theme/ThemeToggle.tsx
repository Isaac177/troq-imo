'use client';

import { Moon, Sun } from 'lucide-react';
import { useThemeToggle } from './useThemeToggle';

type ThemeToggleProps = { lightLabel: string; darkLabel: string };

export function ThemeToggle({ lightLabel, darkLabel }: ThemeToggleProps) {
  const { isDark, label, toggle } = useThemeToggle(lightLabel, darkLabel);

  return (
    <button type="button" onClick={toggle} className="mungini-theme-toggle" aria-label={label} title={label}>
      {isDark ? <Sun size={18} /> : <Moon size={18} />}
    </button>
  );
}
