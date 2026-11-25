import type { ReactNode } from 'react';

type ShellHeaderProps = {
  title: string;
  actions?: ReactNode;
};

export function ShellHeader({ title, actions }: ShellHeaderProps) {
  return (
    <header className="mungini-shell-header">
      <h1>{title}</h1>
      <div className="mungini-shell-actions">{actions}</div>
    </header>
  );
}
