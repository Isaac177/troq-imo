import type { ComponentProps } from 'react';
import { cva, type VariantProps } from 'class-variance-authority';
import { cn } from '../lib/cn';

const skeletonVariants = cva('animate-pulse bg-muted motion-reduce:animate-none', {
  variants: {
    shape: {
      line: 'h-3 w-full rounded-md',
      block: 'h-20 w-full rounded-lg',
      circle: 'size-8 rounded-full',
      row: 'h-10 w-full rounded-md',
    },
  },
  defaultVariants: { shape: 'line' },
});

type SkeletonProps = Omit<ComponentProps<'div'>, 'children'> & VariantProps<typeof skeletonVariants>;

export const Skeleton = ({ shape, className, ...rest }: SkeletonProps) => <div {...rest} aria-hidden="true" className={cn(skeletonVariants({ shape }), className)} />;
