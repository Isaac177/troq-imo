export const modalScrimClasses = 'fixed inset-0 z-50 bg-scrim/40 backdrop-blur-[1px] data-[state=open]:animate-fade-in data-[state=closed]:animate-fade-out';

export const sheetScrimClasses = 'fixed inset-0 z-50 bg-background/60 data-[state=open]:animate-fade-in data-[state=closed]:animate-fade-out';

export const surfaceClasses = 'border border-border bg-popover text-popover-foreground shadow-elevated outline-none';

export const dialogSizes = {
  sm: 'max-w-[400px]',
  md: 'max-w-[520px]',
  lg: 'max-w-[720px]',
} as const;

export type DialogSize = keyof typeof dialogSizes;
