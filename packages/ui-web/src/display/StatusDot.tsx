import { statusDotTones, type StatusTone } from '@mungini/theme/tones';
import { cn } from '../lib/cn';

type StatusDotProps = {
  tone: StatusTone;
  label: string;
  className?: string;
};

export const StatusDot = ({ tone, label, className }: StatusDotProps) => (
  <span className={cn('inline-flex items-center gap-2 whitespace-nowrap text-body text-foreground', className)}>
    <span className={cn('size-2 shrink-0 rounded-full', statusDotTones[tone])} aria-hidden="true" />
    {label}
  </span>
);
