import type { ReactNode } from 'react';
import { X } from 'lucide-react';
import { cn } from '../lib/cn';
import { IconTooltipButton } from '../button/IconTooltipButton';
import { useUiCopy } from '../copy/useUiCopy';

type BulkActionBarProps = {
  selectedCount: number;
  onClear: () => void;
  children: ReactNode;
  className?: string;
};

export const BulkActionBar = ({ selectedCount, onClear, children, className }: BulkActionBarProps) => {
  const { bulk } = useUiCopy();
  const open = selectedCount > 0;
  return (
    <div
      inert={!open}
      role="region"
      aria-live="polite"
      className={cn(
        'sticky bottom-3 z-10 mx-auto flex w-fit max-w-full items-center gap-2 rounded-card border border-border bg-popover px-3 py-2 shadow-elevated transition-all duration-200 ease-in-out',
        open ? 'translate-y-0 opacity-100' : 'pointer-events-none translate-y-2 opacity-0',
        className,
      )}
    >
      <span className="whitespace-nowrap text-body font-medium tabular-nums text-foreground">{bulk.selected(selectedCount)}</span>
      <div className="flex items-center gap-1">{children}</div>
      <IconTooltipButton icon={<X />} label={bulk.clearSelection} onClick={onClear} />
    </div>
  );
};
