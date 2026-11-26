import { createContext } from 'react';
import type { SessionContextValue } from './context';

export const SessionContext = createContext<SessionContextValue | null>(null);
