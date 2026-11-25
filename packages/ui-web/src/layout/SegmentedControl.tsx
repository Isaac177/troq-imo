import { cn } from '../lib/cn';
import { useArrowNavigation } from '../lib/useArrowNavigation';

export type SegmentedItem = { value: string; label: string };

type SegmentedControlProps = {
  items: SegmentedItem[];
  value: string;
  onChange: (value: string) => void;
  ariaLabel: string;
  size?: 'sm' | 'md';
  className?: string;
};

export const SegmentedControl = ({ items, value, onChange, ariaLabel, size = 'md', className }: SegmentedControlProps) => {
  const onKeyDown = useArrowNavigation({ values: items.map((item) => item.value), value, onChange, selector: '[role="radio"]' });
  return (
    <div role="radiogroup" aria-label={ariaLabel} onKeyDown={onKeyDown} className={cn('no-scrollbar inline-flex max-w-full gap-0.5 overflow-x-auto rounded-lg bg-muted p-0.5', className)}>
      {items.map((item) => {
        const active = item.value === value;
        return (
          <button
            key={item.value}
            type="button"
            role="radio"
            aria-checked={active}
            tabIndex={active ? 0 : -1}
            onClick={() => onChange(item.value)}
            className={cn(
              'shrink-0 whitespace-nowrap rounded-md px-3 text-body font-medium transition-all duration-200 ease-in-out focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring',
              size === 'md' ? 'h-8' : 'h-6',
              active ? 'bg-card text-foreground shadow-segment' : 'text-muted-foreground hover:bg-card/60',
            )}
          >
            {item.label}
          </button>
        );
      })}
    </div>
  );
};
