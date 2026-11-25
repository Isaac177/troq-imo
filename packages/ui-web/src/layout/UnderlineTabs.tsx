import { cn } from '../lib/cn';
import { useArrowNavigation } from '../lib/useArrowNavigation';
import { Badge } from '../display/Badge';

export type UnderlineTabItem = { value: string; label: string; count?: number };

type UnderlineTabsProps = {
  items: UnderlineTabItem[];
  value: string;
  onChange: (value: string) => void;
  ariaLabel: string;
  className?: string;
};

export const UnderlineTabs = ({ items, value, onChange, ariaLabel, className }: UnderlineTabsProps) => {
  const onKeyDown = useArrowNavigation({ values: items.map((item) => item.value), value, onChange, selector: '[role="tab"]' });
  return (
    <div role="tablist" aria-label={ariaLabel} onKeyDown={onKeyDown} className={cn('no-scrollbar flex min-w-0 gap-4 overflow-x-auto border-b border-border', className)}>
      {items.map((item) => {
        const active = item.value === value;
        return (
          <button
            key={item.value}
            type="button"
            role="tab"
            aria-selected={active}
            tabIndex={active ? 0 : -1}
            onClick={() => onChange(item.value)}
            className={cn(
              '-mb-px inline-flex h-9 shrink-0 items-center gap-1.5 whitespace-nowrap border-b-2 text-body font-medium transition-colors duration-200 ease-in-out focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring',
              active ? 'border-primary text-foreground' : 'border-transparent text-muted-foreground hover:text-foreground',
            )}
          >
            {item.label}
            {item.count === undefined ? null : <Badge>{item.count}</Badge>}
          </button>
        );
      })}
    </div>
  );
};
