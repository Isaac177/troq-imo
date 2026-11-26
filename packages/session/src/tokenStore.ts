export type StoredTokens = {
  accessToken: string;
  refreshToken: string;
};

export type StoredState = StoredTokens & {
  city: string | null;
};

export type TokenStore = {
  read: () => StoredState;
  writeTokens: (tokens: StoredTokens) => void;
  writeCity: (city: string | null) => void;
  clear: () => void;
};

const empty: StoredState = { accessToken: '', refreshToken: '', city: null };

const safe = (action: () => void) => {
  try {
    action();
  } catch {
    return;
  }
};

export const browserTokenStore = (prefix: string): TokenStore => {
  const accessKey = `${prefix}:accessToken`;
  const refreshKey = `${prefix}:refreshToken`;
  const cityKey = `${prefix}:city`;

  const read = (): StoredState => {
    try {
      return {
        accessToken: localStorage.getItem(accessKey) ?? '',
        refreshToken: localStorage.getItem(refreshKey) ?? '',
        city: localStorage.getItem(cityKey),
      };
    } catch {
      return empty;
    }
  };

  return {
    read,
    writeTokens: (tokens) =>
      safe(() => {
        localStorage.setItem(accessKey, tokens.accessToken);
        localStorage.setItem(refreshKey, tokens.refreshToken);
      }),
    writeCity: (city) =>
      safe(() => {
        if (city) localStorage.setItem(cityKey, city);
        else localStorage.removeItem(cityKey);
      }),
    clear: () =>
      safe(() => {
        localStorage.removeItem(accessKey);
        localStorage.removeItem(refreshKey);
        localStorage.removeItem(cityKey);
      }),
  };
};
