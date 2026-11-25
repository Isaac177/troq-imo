export const hideBelowClasses = {
  sm: 'hidden sm:table-cell',
  md: 'hidden md:table-cell',
  lg: 'hidden lg:table-cell',
} as const;

export const stickyCellClasses =
  'sticky z-[1] bg-card bg-linear-to-r from-transparent to-transparent transition-colors duration-200 ease-in-out group-hover/row:from-muted/50 group-hover/row:to-muted/50';

export const stickyHeadClasses = 'sticky z-[1] bg-card bg-linear-to-r from-muted/40 to-muted/40';

export const numberColumnClasses = 'w-10 pl-3 pr-2 text-right tabular-nums text-muted-foreground md:w-14 md:pl-4';

export const selectColumnClasses = 'w-8 px-2 md:w-10';
