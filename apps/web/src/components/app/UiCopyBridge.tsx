import type { ReactNode } from 'react';
import { UiCopyProvider } from '@mungini/ui-web';
import { useUiCopyBridge } from '@/hooks/layout/useUiCopyBridge';

export default function UiCopyBridge({ children }: { children: ReactNode }) {
  const copy = useUiCopyBridge();
  return <UiCopyProvider copy={copy}>{children}</UiCopyProvider>;
}
