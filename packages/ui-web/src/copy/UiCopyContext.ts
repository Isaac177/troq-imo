import { createContext } from 'react';
import type { UiCopy } from './UiCopy';

export const UiCopyContext = createContext<UiCopy | null>(null);
