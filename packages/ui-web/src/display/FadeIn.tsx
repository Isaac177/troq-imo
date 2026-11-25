import type { ReactNode } from 'react';
import { cn } from '../lib/cn';

type FadeInProps = {
  swapKey: string | number;
  children: ReactNode;
  className?: string;
};

export const FadeIn = ({ swapKey, children, className }: FadeInProps) => (
  <div key={swapKey} className={cn('animate-fade-in', className)}>
    {children}
  </div>
);
