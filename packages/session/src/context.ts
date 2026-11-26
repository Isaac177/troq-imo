import type { components } from '@mungini/api';
import type { SessionClient } from './client';
import type { SessionView } from './types';

export type RegisterInput = components['schemas']['RegisterRequest'];

export type SessionStatus = 'loading' | 'anonymous' | 'authenticated';

export type SessionContextValue = {
  status: SessionStatus;
  session: SessionView | null;
  client: SessionClient;
  error: string | null;
  busy: boolean;
  login: (identifier: string, password: string) => Promise<string | null>;
  register: (input: RegisterInput) => Promise<string | null>;
  logout: () => Promise<void>;
  reload: () => Promise<void>;
  setCity: (city: string) => Promise<void>;
  city: string;
  can: (permission: string, hostId?: string | null) => boolean;
};
