import { createApiClient } from '@mungini/api';
import type { TokenStore } from './tokenStore';
import type { TokenPair } from './types';

type Client = ReturnType<typeof createApiClient>;

export type SessionClient = {
  api: Client;
  store: TokenStore;
  login: (identifier: string, password: string) => Promise<string | null>;
  refresh: () => Promise<string | null>;
  storeTokens: (tokens: TokenPair) => void;
  clear: () => void;
  city: () => string | null;
  setCity: (city: string | null) => void;
};

const decodeExpiry = (token: string): number | null => {
  const payload = token.split('.')[1];
  if (!payload) return null;
  try {
    const normalized = payload.replace(/-/g, '+').replace(/_/g, '/');
    const decoded = JSON.parse(atob(normalized)) as { exp?: number };
    return typeof decoded.exp === 'number' ? decoded.exp : null;
  } catch {
    return null;
  }
};

const expiring = (token: string) => {
  const expiry = decodeExpiry(token);
  return expiry !== null && expiry - 30 < Math.floor(Date.now() / 1000);
};

export const createSessionClient = (options: { baseUrl: string; store: TokenStore }): SessionClient => {
  const { store } = options;
  let refreshing: Promise<string | null> | null = null;

  const api = createApiClient({ baseUrl: options.baseUrl.replace(/\/$/, '') });

  const refresh = (): Promise<string | null> => {
    if (refreshing) return refreshing;
    const current = store.read();
    if (!current.refreshToken) return Promise.resolve(null);
    refreshing = (async () => {
      const response = await fetch(`${options.baseUrl.replace(/\/$/, '')}/api/public/auth/refresh`, {
        method: 'POST',
        headers: { 'content-type': 'application/json', 'x-city': current.city ?? 'GOMA' },
        body: JSON.stringify({ refreshToken: current.refreshToken }),
      });
      if (!response.ok) {
        store.clear();
        return null;
      }
      const body = (await response.json()) as { data?: TokenPair };
      if (!body.data?.accessToken || !body.data.refreshToken) {
        store.clear();
        return null;
      }
      store.writeTokens(body.data);
      return body.data.accessToken;
    })().finally(() => {
      refreshing = null;
    });
    return refreshing;
  };

  api.use({
    async onRequest({ request }) {
      const current = store.read();
      let token = current.accessToken;
      if (token && expiring(token)) {
        token = (await refresh()) ?? token;
      }
      if (token) request.headers.set('authorization', `Bearer ${token}`);
      if (current.city) request.headers.set('x-city', current.city);
      return request;
    },
    async onResponse({ request, response }) {
      if (response.status !== 401 || request.method !== 'GET') return response;
      const token = await refresh();
      if (!token) return response;
      const retry = new Request(request, { headers: new Headers(request.headers) });
      retry.headers.set('authorization', `Bearer ${token}`);
      return fetch(retry);
    },
  });

  return {
    api,
    store,
    refresh,
    city: () => store.read().city,
    setCity: (city) => store.writeCity(city),
    storeTokens: (tokens) => store.writeTokens(tokens),
    clear: () => store.clear(),
    async login(identifier, password) {
      const { data, error } = await api.POST('/api/public/auth/login', {
        body: { identifier, password },
      });
      if (error || !data) return error?.error.code ?? 'INTERNAL_ERROR';
      store.writeTokens(data.data);
      return null;
    },
  };
};
