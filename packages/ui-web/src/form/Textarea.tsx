import type { ComponentProps } from 'react';
import { cn } from '../lib/cn';
import { controlClasses } from '../lib/fieldClasses';

export type TextareaProps = ComponentProps<'textarea'> & { invalid?: boolean };

export const Textarea = ({ invalid, className, rows = 3, ...rest }: TextareaProps) => (
  <textarea {...rest} rows={rows} aria-invalid={invalid || undefined} className={cn(controlClasses, 'h-auto min-h-[72px] resize-y py-2', className)} />
);
