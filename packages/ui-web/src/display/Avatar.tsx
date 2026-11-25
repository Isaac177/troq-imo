import * as AvatarPrimitive from '@radix-ui/react-avatar';
import { cva, type VariantProps } from 'class-variance-authority';
import { cn } from '../lib/cn';
import { useInitials } from './useInitials';

const avatarVariants = cva('relative flex shrink-0 overflow-hidden rounded-full bg-muted', {
  variants: {
    size: {
      sm: 'size-6 text-micro',
      md: 'size-8 text-label',
      lg: 'size-9 text-body',
    },
  },
  defaultVariants: { size: 'md' },
});

type AvatarProps = VariantProps<typeof avatarVariants> & {
  name: string;
  src?: string | null;
  className?: string;
};

export const Avatar = ({ name, src, size, className }: AvatarProps) => {
  const initials = useInitials(name);
  return (
    <AvatarPrimitive.Root className={cn(avatarVariants({ size }), className)}>
      {src ? <AvatarPrimitive.Image src={src} alt={name} className="size-full object-cover" /> : null}
      <AvatarPrimitive.Fallback className="flex size-full items-center justify-center font-medium text-muted-foreground" aria-label={name}>
        {initials}
      </AvatarPrimitive.Fallback>
    </AvatarPrimitive.Root>
  );
};
