import { useContext } from 'react';
import type { SessionContextValue } from './context';
import { SessionContext } from './sessionContext';

export const useSession = (): SessionContextValue => {
  const value = useContext(SessionContext);
  if (!value) throw new Error('useSession must be used inside SessionProvider');
  return value;
};
