import type { ComponentProps, ReactNode } from 'react';
import { Slot } from '@radix-ui/react-slot';
import { Loader2 } from 'lucide-react';
import type { ButtonVariantProps } from './buttonVariants';
import { useButton } from './useButton';

export type ButtonProps = Omit<ComponentProps<'button'>, 'title'> &
  ButtonVariantProps & {
    asChild?: boolean;
    icon?: ReactNode;
    collapseLabel?: boolean;
    loading?: boolean;
    title?: string;
  };

export const Button = ({ asChild = false, variant, size, className, children, icon, collapseLabel, loading, disabled, title, type = 'button', ...rest }: ButtonProps) => {
  const view = useButton({ variant, size, className, children, collapseLabel, loading, disabled, ariaLabel: rest['aria-label'], title });
  if (asChild) {
    return (
      <Slot className={view.className} {...rest}>
        {children}
      </Slot>
    );
  }
  return (
    <button {...rest} type={type} className={view.className} disabled={view.disabled} aria-label={view.ariaLabel} aria-busy={loading || undefined} title={view.title}>
      {loading ? <Loader2 className="animate-spin" aria-hidden="true" /> : icon}
      {view.collapsed ? <span className="hidden sm:inline">{children}</span> : children}
    </button>
  );
};
