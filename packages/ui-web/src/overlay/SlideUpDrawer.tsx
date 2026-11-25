import type { ReactNode } from 'react';
import * as DialogPrimitive from '@radix-ui/react-dialog';
import { cn } from '../lib/cn';
import { OverlayHeader } from './OverlayHeader';
import { sheetScrimClasses, surfaceClasses } from './overlayClasses';

type SlideUpDrawerProps = {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  title: string;
  description?: string;
  children?: ReactNode;
  footer?: ReactNode;
  className?: string;
};

export const SlideUpDrawer = ({ open, onOpenChange, title, description, children, footer, className }: SlideUpDrawerProps) => (
  <DialogPrimitive.Root open={open} onOpenChange={onOpenChange}>
    <DialogPrimitive.Portal>
      <DialogPrimitive.Overlay className={sheetScrimClasses} />
      <DialogPrimitive.Content
        className={cn(
          'fixed inset-x-0 bottom-0 z-50 mx-auto flex h-[92dvh] w-full flex-col rounded-b-none rounded-t-card data-[state=open]:animate-sheet-in data-[state=closed]:animate-sheet-out md:max-w-[720px]',
          surfaceClasses,
          className,
        )}
      >
        <OverlayHeader title={title} description={description} />
        {children ? <div className="min-h-0 flex-1 overflow-y-auto px-3 pb-3 md:px-5 md:pb-4">{children}</div> : null}
        {footer ? <div className="flex shrink-0 flex-wrap justify-end gap-2 border-t border-border px-3 py-3 md:px-5">{footer}</div> : null}
      </DialogPrimitive.Content>
    </DialogPrimitive.Portal>
  </DialogPrimitive.Root>
);
