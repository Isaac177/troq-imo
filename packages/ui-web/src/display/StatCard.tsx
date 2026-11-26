import type { ReactNode } from 'react';
import { cn } from '../lib/cn';

type StatCardProps = {
  label: string;
  value: string;
  hint?: string;
  icon?: ReactNode;
  className?: string;
};

export const StatCard = ({ label, value, hint, icon, className }: StatCardProps) => (
  <div className={cn('rounded-card border border-border bg-card px-3 py-2.5 transition-colors duration-200 ease-in-out', className)}>
    <div className="flex items-center justify-between gap-2">
      <p className="text-label text-muted-foreground">{label}</p>
      {icon ? <span className="text-muted-foreground [&_svg]:size-4" aria-hidden="true">{icon}</span> : null}
    </div>
    <p className="mt-1 text-[15px] font-semibold tabular-nums text-foreground md:text-title">{value}</p>
    {hint ? <p className="mt-0.5 text-micro text-muted-foreground">{hint}</p> : null}
  </div>
);
