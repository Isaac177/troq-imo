import type { ReactNode } from 'react';
import * as DialogPrimitive from '@radix-ui/react-dialog';
import { cn } from '../lib/cn';
import { OverlayHeader } from './OverlayHeader';
import { dialogSizes, modalScrimClasses, surfaceClasses, type DialogSize } from './overlayClasses';

type DialogProps = {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  title: string;
  description?: string;
  children?: ReactNode;
  footer?: ReactNode;
  size?: DialogSize;
  className?: string;
};

export const Dialog = ({ open, onOpenChange, title, description, children, footer, size = 'md', className }: DialogProps) => (
  <DialogPrimitive.Root open={open} onOpenChange={onOpenChange}>
    <DialogPrimitive.Portal>
      <DialogPrimitive.Overlay className={modalScrimClasses} />
      <DialogPrimitive.Content
        className={cn(
          'fixed left-1/2 top-1/2 z-50 flex max-h-[92dvh] w-[calc(100%-2rem)] -translate-x-1/2 -translate-y-1/2 flex-col rounded-card data-[state=open]:animate-dialog-in data-[state=closed]:animate-dialog-out',
          surfaceClasses,
          dialogSizes[size],
          className,
        )}
      >
        <OverlayHeader title={title} description={description} />
        {children ? <div className="min-h-0 flex-1 overflow-y-auto px-3 pb-3 md:px-5 md:pb-4">{children}</div> : null}
        {footer ? <div className="flex shrink-0 flex-wrap justify-end gap-2 px-3 pb-3 md:px-5 md:pb-4">{footer}</div> : null}
      </DialogPrimitive.Content>
    </DialogPrimitive.Portal>
  </DialogPrimitive.Root>
);
