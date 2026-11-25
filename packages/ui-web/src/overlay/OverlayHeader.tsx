import type { ReactNode } from 'react';
import * as DialogPrimitive from '@radix-ui/react-dialog';
import { ChevronLeft, X } from 'lucide-react';
import { IconTooltipButton } from '../button/IconTooltipButton';
import { useUiCopy } from '../copy/useUiCopy';

type OverlayHeaderProps = {
  title: string;
  description?: string;
  onBack?: () => void;
  actions?: ReactNode;
};

export const OverlayHeader = ({ title, description, onBack, actions }: OverlayHeaderProps) => {
  const { common } = useUiCopy();
  return (
    <div className="flex shrink-0 items-start gap-2 px-3 py-3 md:px-5 md:py-4">
      {onBack ? <IconTooltipButton icon={<ChevronLeft />} label={common.back} onClick={onBack} /> : null}
      <div className="min-w-0 flex-1">
        <DialogPrimitive.Title className="text-title font-semibold text-foreground">{title}</DialogPrimitive.Title>
        {description ? (
          <DialogPrimitive.Description className="mt-0.5 text-body text-muted-foreground">{description}</DialogPrimitive.Description>
        ) : (
          <DialogPrimitive.Description className="sr-only">{title}</DialogPrimitive.Description>
        )}
      </div>
      {actions}
      <DialogPrimitive.Close asChild>
        <IconTooltipButton icon={<X />} label={common.close} />
      </DialogPrimitive.Close>
    </div>
  );
};
