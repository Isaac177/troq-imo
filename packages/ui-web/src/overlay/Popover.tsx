import type { ReactNode } from 'react';
import * as PopoverPrimitive from '@radix-ui/react-popover';
import { cn } from '../lib/cn';
import { surfaceClasses } from './overlayClasses';

type PopoverProps = {
  trigger: ReactNode;
  children: ReactNode;
  open?: boolean;
  onOpenChange?: (open: boolean) => void;
  align?: 'start' | 'center' | 'end';
  side?: 'top' | 'right' | 'bottom' | 'left';
  className?: string;
};

export const Popover = ({ trigger, children, open, onOpenChange, align = 'start', side = 'bottom', className }: PopoverProps) => (
  <PopoverPrimitive.Root open={open} onOpenChange={onOpenChange}>
    <PopoverPrimitive.Trigger asChild>{trigger}</PopoverPrimitive.Trigger>
    <PopoverPrimitive.Portal>
      <PopoverPrimitive.Content
        align={align}
        side={side}
        sideOffset={4}
        collisionPadding={8}
        className={cn(
          'z-50 w-72 max-w-[calc(100vw-1rem)] origin-(--radix-popover-content-transform-origin) rounded-card p-3 data-[state=open]:animate-pop-in data-[state=closed]:animate-pop-out',
          surfaceClasses,
          className,
        )}
      >
        {children}
      </PopoverPrimitive.Content>
    </PopoverPrimitive.Portal>
  </PopoverPrimitive.Root>
);
