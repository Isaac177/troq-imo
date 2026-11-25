import type { ComponentProps, ReactNode } from 'react';
import { Button } from './Button';
import type { ButtonVariantProps } from './buttonVariants';

type IconTooltipButtonProps = Omit<ComponentProps<'button'>, 'title' | 'aria-label' | 'children'> &
  Pick<ButtonVariantProps, 'variant'> & {
    icon: ReactNode;
    label: string;
    loading?: boolean;
  };

export const IconTooltipButton = ({ icon, label, variant = 'ghost', loading, ...rest }: IconTooltipButtonProps) => (
  <Button {...rest} variant={variant} size="icon" loading={loading} aria-label={label} title={label} icon={icon} />
);
