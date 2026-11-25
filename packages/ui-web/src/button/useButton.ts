import type { ReactNode } from 'react';
import { buttonVariants, type ButtonVariantProps } from './buttonVariants';
import { cn } from '../lib/cn';

type UseButtonOptions = ButtonVariantProps & {
  className?: string;
  children?: ReactNode;
  collapseLabel?: boolean;
  loading?: boolean;
  disabled?: boolean;
  ariaLabel?: string;
  title?: string;
};

export const useButton = ({ variant, size, className, children, collapseLabel, loading, disabled, ariaLabel, title }: UseButtonOptions) => {
  const text = typeof children === 'string' ? children : undefined;
  const collapsed = collapseLabel === true;
  return {
    className: cn(buttonVariants({ variant, size }), collapsed && 'size-8 px-0 sm:h-9 sm:w-auto sm:px-3 md:px-4', className),
    disabled: disabled || loading,
    ariaLabel: ariaLabel ?? (collapsed ? text : undefined),
    title: title ?? (collapsed ? text : undefined),
    collapsed,
  };
};
