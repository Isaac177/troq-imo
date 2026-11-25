import type { ReactNode } from 'react';

export type ActionItem = {
  key: string;
  label: string;
  icon?: ReactNode;
  onSelect: () => void;
  destructive?: boolean;
  disabled?: boolean;
  separatorBefore?: boolean;
};
