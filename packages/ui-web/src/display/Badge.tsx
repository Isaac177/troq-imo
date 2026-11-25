import type { ComponentProps } from 'react';
import { cva, type VariantProps } from 'class-variance-authority';
import { cn } from '../lib/cn';

const badgeVariants = cva('inline-flex h-5 min-w-5 items-center justify-center whitespace-nowrap rounded-full px-1.5 text-label font-medium tabular-nums transition-colors duration-200 ease-in-out', {
  variants: {
    variant: {
      secondary: 'bg-secondary text-secondary-foreground',
      primary: 'bg-primary text-primary-foreground',
      outline: 'border border-border text-foreground',
      destructive: 'bg-destructive text-destructive-foreground',
    },
  },
  defaultVariants: { variant: 'secondary' },
});

type BadgeProps = ComponentProps<'span'> & VariantProps<typeof badgeVariants>;

export const Badge = ({ variant, className, ...rest }: BadgeProps) => <span {...rest} className={cn(badgeVariants({ variant }), className)} />;
