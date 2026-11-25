import { CircleAlert, CircleCheck, Info, TriangleAlert, X } from 'lucide-react';
import { cn } from '../lib/cn';
import { IconTooltipButton } from '../button/IconTooltipButton';
import { useUiCopy } from '../copy/useUiCopy';
import type { ToastData, ToastTone } from './toastStore';
import { useToastTimer } from './useToastTimer';

const toneIcons = {
  neutral: { Icon: Info, className: 'text-muted-foreground' },
  success: { Icon: CircleCheck, className: 'text-success' },
  warning: { Icon: TriangleAlert, className: 'text-warning' },
  danger: { Icon: CircleAlert, className: 'text-destructive' },
} as const satisfies Record<ToastTone, { Icon: typeof Info; className: string }>;

export const ToastItem = ({ id, title, description, tone, durationMs }: ToastData) => {
  const { toast } = useUiCopy();
  const timer = useToastTimer(id, durationMs);
  const { Icon, className } = toneIcons[tone];
  return (
    <div
      role={tone === 'danger' ? 'alert' : 'status'}
      onMouseEnter={timer.pause}
      onMouseLeave={timer.resume}
      className="pointer-events-auto flex w-full items-start gap-2 rounded-card border border-border bg-popover p-3 text-popover-foreground shadow-elevated animate-toast-in"
    >
      <Icon className={cn('mt-0.5 size-4 shrink-0', className)} aria-hidden="true" />
      <div className="min-w-0 flex-1">
        <p className="text-body font-medium text-foreground">{title}</p>
        {description ? <p className="mt-0.5 text-label text-muted-foreground">{description}</p> : null}
      </div>
      <IconTooltipButton icon={<X />} label={toast.dismiss} onClick={timer.dismiss} className="-my-1 -mr-1" />
    </div>
  );
};
