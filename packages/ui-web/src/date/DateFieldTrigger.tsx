import type { ComponentProps } from 'react';
import { CalendarDays } from 'lucide-react';
import { cn } from '../lib/cn';
import { controlClasses } from '../lib/fieldClasses';

type DateFieldTriggerProps = Omit<ComponentProps<'button'>, 'children'> & {
  text: string;
  placeholder: string;
  invalid?: boolean;
};

export const DateFieldTrigger = ({ text, placeholder, invalid, className, ...rest }: DateFieldTriggerProps) => (
  <button type="button" {...rest} aria-invalid={invalid || undefined} className={cn(controlClasses, 'flex items-center gap-2 text-left tabular-nums', className)}>
    <CalendarDays className="size-4 shrink-0 text-muted-foreground" aria-hidden="true" />
    <span className={cn('truncate', text === '' && 'text-muted-foreground')}>{text === '' ? placeholder : text}</span>
  </button>
);
