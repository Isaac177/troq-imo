import type { ReactNode } from 'react';
import { cn } from '../lib/cn';

type EmptyStateProps = {
  title: string;
  description?: string;
  icon?: ReactNode;
  action?: ReactNode;
  className?: string;
};

export const EmptyState = ({ title, description, icon, action, className }: EmptyStateProps) => (
  <div className={cn('flex flex-col items-center justify-center gap-2 px-4 py-10 text-center', className)}>
    {icon ? <div className="flex size-9 items-center justify-center rounded-full bg-muted text-muted-foreground [&_svg]:size-4" aria-hidden="true">{icon}</div> : null}
    <p className="text-body font-medium text-foreground">{title}</p>
    {description ? <p className="max-w-[320px] text-label text-muted-foreground">{description}</p> : null}
    {action ? <div className="mt-1">{action}</div> : null}
  </div>
);
