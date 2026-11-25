import { cva, type VariantProps } from 'class-variance-authority';

export const buttonVariants = cva(
  'inline-flex shrink-0 items-center justify-center gap-2 whitespace-nowrap rounded-lg text-body font-medium transition-colors duration-200 ease-in-out focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0',
  {
    variants: {
      variant: {
        primary: 'bg-primary text-primary-foreground hover:bg-primary-hover',
        outline: 'border border-border bg-background text-foreground hover:bg-muted',
        ghost: 'text-foreground hover:bg-muted',
        secondary: 'bg-secondary text-secondary-foreground hover:bg-muted',
        destructive: 'bg-destructive text-destructive-foreground hover:bg-destructive/90',
        link: 'h-auto px-0 text-tone-identity underline-offset-4 hover:underline',
      },
      size: {
        default: 'h-9 px-3 md:px-4',
        sm: 'h-8 px-3',
        icon: 'size-8',
        iconLg: 'size-9',
      },
    },
    defaultVariants: {
      variant: 'primary',
      size: 'default',
    },
  },
);

export type ButtonVariantProps = VariantProps<typeof buttonVariants>;
