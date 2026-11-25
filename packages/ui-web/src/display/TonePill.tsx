import { statusDotTones, statusPillTones, type StatusTone } from '@mungini/theme/tones';
import { cn } from '../lib/cn';

type TonePillProps = {
  tone: StatusTone;
  label: string;
  className?: string;
};

export const TonePill = ({ tone, label, className }: TonePillProps) => (
  <span className={cn('inline-flex h-6 items-center gap-1.5 whitespace-nowrap rounded-full px-2 text-label font-medium text-foreground', statusPillTones[tone], className)}>
    <span className={cn('size-1.5 shrink-0 rounded-full', statusDotTones[tone])} aria-hidden="true" />
    {label}
  </span>
);
