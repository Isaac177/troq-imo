import { useId } from 'react';
import * as SwitchPrimitive from '@radix-ui/react-switch';
import { cn } from '../lib/cn';

type SwitchProps = {
  checked: boolean;
  onCheckedChange: (checked: boolean) => void;
  label?: string;
  ariaLabel?: string;
  disabled?: boolean;
  className?: string;
};

export const Switch = ({ checked, onCheckedChange, label, ariaLabel, disabled, className }: SwitchProps) => {
  const id = useId();
  const control = (
    <SwitchPrimitive.Root
      id={id}
      checked={checked}
      disabled={disabled}
      aria-label={label ? undefined : ariaLabel}
      onCheckedChange={onCheckedChange}
      className={cn(
        'inline-flex h-5 w-9 shrink-0 items-center rounded-full bg-input p-0.5 transition-colors duration-200 ease-in-out focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-primary',
        !label && className,
      )}
    >
      <SwitchPrimitive.Thumb className="block size-4 rounded-full bg-background transition-transform duration-200 ease-in-out data-[state=checked]:translate-x-4" />
    </SwitchPrimitive.Root>
  );
  if (!label) return control;
  return (
    <label htmlFor={id} className={cn('inline-flex min-h-8 items-center gap-2 text-body text-foreground', className)}>
      {control}
      {label}
    </label>
  );
};
