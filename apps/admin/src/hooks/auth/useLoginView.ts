import { useState } from 'react';
import { useSession } from '@mungini/session';
import { useTranslation } from '@mungini/i18n';

export const useLoginView = () => {
  const { t } = useTranslation('auth');
  const { login, status, busy } = useSession();
  const [identifier, setIdentifier] = useState('');
  const [password, setPassword] = useState('');
  const [error, setError] = useState<string | null>(null);

  const message = (code: string | null) => {
    if (!code) return null;
    if (code === 'INVALID_CREDENTIALS') return t('invalidCredentials');
    if (code === 'ACCOUNT_BLOCKED') return t('accountBlocked');
    return t('errorGeneric');
  };

  const submit = async (event: React.FormEvent) => {
    event.preventDefault();
    setError(null);
    const code = await login(identifier.trim(), password);
    setError(message(code));
  };

  return {
    identifier,
    password,
    setIdentifier,
    setPassword,
    submit,
    error,
    busy,
    authenticated: status === 'authenticated',
    loading: status === 'loading',
    labels: {
      title: t('signIn'),
      subtitle: t('signInSubtitle'),
      identifier: t('identifier'),
      identifierPlaceholder: t('identifierPlaceholder'),
      password: t('password'),
      passwordPlaceholder: t('passwordPlaceholder'),
      submit: t('submit'),
      signingIn: t('signingIn'),
      loading: t('loading'),
      demoHint: t('demoHint'),
    },
  };
};
