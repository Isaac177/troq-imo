import { cn } from '../lib/cn';
import { menuItemClasses, popupClasses } from '../lib/fieldClasses';

export const menuContentClasses = cn(popupClasses, 'min-w-[180px] rounded-card p-1 data-[state=open]:animate-pop-in data-[state=closed]:animate-pop-out');

export const menuRowClasses = (destructive: boolean | undefined): string => cn(menuItemClasses, destructive && 'text-destructive');

export const menuSeparatorClasses = '-mx-1 my-1 h-px bg-border';
