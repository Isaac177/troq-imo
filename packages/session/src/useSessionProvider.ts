import { useCallback, useEffect, useRef, useState } from 'react';
import { apiErrorCode } from './errors';
import type { RegisterInput, SessionContextValue, SessionStatus } from './context';
import type { SessionClient } from './client';
import type { SessionView } from './types';

const DEFAULT_CITY = 'GOMA';

export const useSessionProvider = (client: SessionClient): SessionContextValue => {
  const [status, setStatus] = useState<SessionStatus>('loading');
  const [session, setSession] = useState<SessionView | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);
  const [city, setCityState] = useState(() => client.city() ?? DEFAULT_CITY);
  const mounted = useRef(true);

  useEffect(() => {
    mounted.current = true;
    return () => {
      mounted.current = false;
    };
  }, []);

  useEffect(() => {
    client.setCity(city);
  }, [client, city]);

  const load = useCallback(async () => {
    const tokens = client.store.read();
    if (!tokens.accessToken) {
      setStatus('anonymous');
      setSession(null);
      return null;
    }
    const { data, error: responseError } = await client.api.GET('/api/guest/session');
    if (data) {
      setSession(data.data);
      setStatus('authenticated');
      setError(null);
      return data.data;
    }
    if (apiErrorCode(responseError) === 'AUTH_REQUIRED' || apiErrorCode(responseError) === 'TOKEN_INVALID' || apiErrorCode(responseError) === 'SESSION_REVOKED') {
      const token = await client.refresh();
      if (token) {
        const retry = await client.api.GET('/api/guest/session');
        if (retry.data) {
          setSession(retry.data.data);
          setStatus('authenticated');
          setError(null);
          return retry.data.data;
        }
      }
      client.clear();
      setSession(null);
      setStatus('anonymous');
      return null;
    }
    setError(apiErrorCode(responseError));
    setStatus('anonymous');
    return null;
  }, [client]);

  useEffect(() => {
    void load();
  }, [load]);

  const login = useCallback(
    async (identifier: string, password: string) => {
      setBusy(true);
      setError(null);
      const code = await client.login(identifier, password);
      if (code) {
        setBusy(false);
        setError(code);
        return code;
      }
      await load();
      setBusy(false);
      return null;
    },
    [client, load],
  );

  const register = useCallback(
    async (input: RegisterInput) => {
      setBusy(true);
      setError(null);
      const { data, error: responseError } = await client.api.POST('/api/public/auth/register', { body: input });
      if (responseError || !data) {
        const code = apiErrorCode(responseError);
        setBusy(false);
        setError(code);
        return code;
      }
      client.storeTokens(data.data);
      await load();
      setBusy(false);
      return null;
    },
    [client, load],
  );

  const logout = useCallback(async () => {
    setBusy(true);
    await client.api.POST('/api/guest/session/logout');
    client.clear();
    setSession(null);
    setStatus('anonymous');
    setBusy(false);
  }, [client]);

  const setCity = useCallback(
    async (next: string) => {
      setCityState(next);
      client.setCity(next);
      if (client.store.read().accessToken) await load();
    },
    [client, load],
  );

  const can = useCallback<SessionContextValue['can']>(
    (permission, hostId) =>
      session?.permissions.some(
        (grant) =>
          grant.permission === permission &&
          (grant.scopeType === 'PLATFORM' || hostId === undefined || hostId === null || grant.scopeId === hostId),
      ) ?? false,
    [session],
  );

  return {
    status,
    session,
    client,
    error,
    busy,
    login,
    register,
    logout,
    reload: async () => {
      await load();
    },
    setCity,
    city,
    can,
  };
};
