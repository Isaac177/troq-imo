import * as SelectPrimitive from '@radix-ui/react-select';
import { Check, ChevronDown } from 'lucide-react';
import { cn } from '../lib/cn';
import { controlClasses, popupClasses } from '../lib/fieldClasses';
import { statusDotTones } from '@mungini/theme/tones';
import type { SelectOption } from './selectOption';

type SelectProps = {
  options: SelectOption[];
  value: string;
  onValueChange: (value: string) => void;
  placeholder?: string;
  ariaLabel?: string;
  id?: string;
  disabled?: boolean;
  invalid?: boolean;
  className?: string;
};

export const Select = ({ options, value, onValueChange, placeholder, ariaLabel, id, disabled, invalid, className }: SelectProps) => (
  <SelectPrimitive.Root value={value === '' ? undefined : value} onValueChange={onValueChange} disabled={disabled}>
    <SelectPrimitive.Trigger
      id={id}
      aria-label={ariaLabel}
      aria-invalid={invalid || undefined}
      className={cn(controlClasses, 'flex items-center justify-between gap-2 text-left data-[placeholder]:text-muted-foreground', className)}
    >
      <SelectPrimitive.Value placeholder={placeholder} />
      <SelectPrimitive.Icon asChild>
        <ChevronDown className="size-4 shrink-0 text-muted-foreground" aria-hidden="true" />
      </SelectPrimitive.Icon>
    </SelectPrimitive.Trigger>
    <SelectPrimitive.Portal>
      <SelectPrimitive.Content
        position="popper"
        sideOffset={4}
        collisionPadding={8}
        className={cn(popupClasses, 'max-h-[min(320px,var(--radix-select-content-available-height))] min-w-(--radix-select-trigger-width) origin-(--radix-select-content-transform-origin) overflow-hidden data-[state=open]:animate-pop-in data-[state=closed]:animate-pop-out')}
      >
        <SelectPrimitive.Viewport className="p-1">
          {options.map((option) => (
            <SelectPrimitive.Item
              key={option.value}
              value={option.value}
              disabled={option.disabled}
              className="relative flex min-h-9 cursor-default select-none items-center gap-2 rounded-md pl-2 pr-8 text-body text-popover-foreground outline-none transition-colors duration-200 ease-in-out data-[disabled]:pointer-events-none data-[disabled]:opacity-50 data-[highlighted]:bg-accent"
            >
              {option.tone ? <span className={cn('size-2 shrink-0 rounded-full', statusDotTones[option.tone])} aria-hidden="true" /> : null}
              <SelectPrimitive.ItemText>{option.label}</SelectPrimitive.ItemText>
              <SelectPrimitive.ItemIndicator className="absolute right-2">
                <Check className="size-4" aria-hidden="true" />
              </SelectPrimitive.ItemIndicator>
            </SelectPrimitive.Item>
          ))}
        </SelectPrimitive.Viewport>
      </SelectPrimitive.Content>
    </SelectPrimitive.Portal>
  </SelectPrimitive.Root>
);
