import type { ReactNode } from 'react';
import * as DialogPrimitive from '@radix-ui/react-dialog';
import { cn } from '../lib/cn';
import { OverlayHeader } from './OverlayHeader';
import { modalScrimClasses, surfaceClasses } from './overlayClasses';

type DrawerProps = {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  title: string;
  description?: string;
  children?: ReactNode;
  footer?: ReactNode;
  modal?: boolean;
  onBack?: () => void;
  className?: string;
};

export const Drawer = ({ open, onOpenChange, title, description, children, footer, modal = true, onBack, className }: DrawerProps) => (
  <DialogPrimitive.Root open={open} onOpenChange={onOpenChange} modal={modal}>
    <DialogPrimitive.Portal>
      {modal ? <DialogPrimitive.Overlay className={modalScrimClasses} /> : null}
      <DialogPrimitive.Content
        onInteractOutside={modal ? undefined : (event) => event.preventDefault()}
        className={cn(
          surfaceClasses,
          'fixed inset-y-0 right-0 z-50 flex w-full flex-col border-y-0 border-r-0 data-[state=open]:animate-drawer-in data-[state=closed]:animate-drawer-out sm:w-[420px]',
          className,
        )}
      >
        <OverlayHeader title={title} description={description} onBack={onBack} />
        {children ? <div className="min-h-0 flex-1 overflow-y-auto px-3 pb-3 md:px-5 md:pb-4">{children}</div> : null}
        {footer ? <div className="flex shrink-0 flex-wrap justify-end gap-2 border-t border-border px-3 py-3 md:px-5">{footer}</div> : null}
      </DialogPrimitive.Content>
    </DialogPrimitive.Portal>
  </DialogPrimitive.Root>
);
