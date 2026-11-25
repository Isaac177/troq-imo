import { useId } from 'react';
import * as CheckboxPrimitive from '@radix-ui/react-checkbox';
import { Check, Minus } from 'lucide-react';
import { cn } from '../lib/cn';

type CheckboxProps = {
  checked: boolean | 'indeterminate';
  onCheckedChange: (checked: boolean) => void;
  label?: string;
  ariaLabel?: string;
  disabled?: boolean;
  className?: string;
};

export const Checkbox = ({ checked, onCheckedChange, label, ariaLabel, disabled, className }: CheckboxProps) => {
  const id = useId();
  const box = (
    <CheckboxPrimitive.Root
      id={id}
      checked={checked}
      disabled={disabled}
      aria-label={label ? undefined : ariaLabel}
      onCheckedChange={(next) => onCheckedChange(next === true)}
      className={cn(
        'flex size-4 shrink-0 items-center justify-center rounded-[4px] border border-input bg-background text-primary-foreground transition-colors duration-200 ease-in-out focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:border-primary data-[state=checked]:bg-primary data-[state=indeterminate]:border-primary data-[state=indeterminate]:bg-primary',
        !label && className,
      )}
    >
      <CheckboxPrimitive.Indicator>{checked === 'indeterminate' ? <Minus className="size-3" /> : <Check className="size-3" />}</CheckboxPrimitive.Indicator>
    </CheckboxPrimitive.Root>
  );
  if (!label) return box;
  return (
    <label htmlFor={id} className={cn('inline-flex min-h-8 items-center gap-2 text-body text-foreground', className)}>
      {box}
      {label}
    </label>
  );
};
