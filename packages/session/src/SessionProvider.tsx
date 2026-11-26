import type { ReactNode } from 'react';
import type { SessionClient } from './client';
import { SessionContext } from './sessionContext';
import { useSessionProvider } from './useSessionProvider';

type SessionProviderProps = {
  client: SessionClient;
  children: ReactNode;
};

export const SessionProvider = ({ client, children }: SessionProviderProps) => {
  const value = useSessionProvider(client);
  return <SessionContext.Provider value={value}>{children}</SessionContext.Provider>;
};
