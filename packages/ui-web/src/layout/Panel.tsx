import type { ReactNode } from 'react';
import { cn } from '../lib/cn';

type PanelProps = {
  header: ReactNode;
  children: ReactNode;
  collapsed?: boolean;
  className?: string;
  bodyClassName?: string;
};

export const Panel = ({ header, children, collapsed = false, className, bodyClassName }: PanelProps) => (
  <section className={cn('flex flex-col rounded-card border border-border bg-muted/40', className)}>
    <div className="flex shrink-0 items-start gap-2 px-3 py-3 md:px-5">
      <div className="min-w-0 flex-1">{header}</div>
    </div>
    <div
      className={cn(
        'grid min-h-0 flex-1 transition-[grid-template-rows,opacity] duration-200 ease-in-out',
        collapsed ? 'grid-rows-[0fr] opacity-0' : 'grid-rows-[1fr] opacity-100',
      )}
    >
      <div className={cn('-mx-px -mb-px min-h-0 overflow-hidden rounded-card border border-border bg-card', collapsed && 'border-transparent', bodyClassName)}>{children}</div>
    </div>
  </section>
);
