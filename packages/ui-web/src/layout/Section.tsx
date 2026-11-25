import type { ReactNode } from 'react';
import { Panel } from './Panel';

type SectionProps = {
  title: string;
  description?: string;
  actions?: ReactNode;
  children: ReactNode;
  className?: string;
};

export const Section = ({ title, description, actions, children, className }: SectionProps) => (
  <Panel
    className={className}
    header={
      <div className="flex items-center justify-between gap-3">
        <div className="min-w-0">
          <h2 className="text-title font-semibold text-foreground">{title}</h2>
          {description ? <p className="mt-0.5 text-body text-muted-foreground">{description}</p> : null}
        </div>
        {actions ? <div className="flex shrink-0 items-center gap-2">{actions}</div> : null}
      </div>
    }
  >
    <div className="space-y-4 p-3 md:p-5">{children}</div>
  </Panel>
);
