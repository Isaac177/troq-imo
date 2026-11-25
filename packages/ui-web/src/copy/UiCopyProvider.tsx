import type { ReactNode } from 'react';
import type { UiCopy } from './UiCopy';
import { UiCopyContext } from './UiCopyContext';

type UiCopyProviderProps = {
  copy: UiCopy;
  children: ReactNode;
};

export const UiCopyProvider = ({ copy, children }: UiCopyProviderProps) => (
  <UiCopyContext.Provider value={copy}>{children}</UiCopyContext.Provider>
);
