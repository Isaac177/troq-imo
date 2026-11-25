import { useContext } from 'react';
import type { UiCopy } from './UiCopy';
import { UiCopyContext } from './UiCopyContext';

export const useUiCopy = (): UiCopy => {
  const copy = useContext(UiCopyContext);
  if (!copy) throw new Error('UiCopyProvider is missing above this component');
  return copy;
};
