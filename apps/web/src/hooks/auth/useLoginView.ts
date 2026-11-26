import { useState } from 'react';
import { useRouter } from 'next/navigation';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';

export const useLoginView = () => {
  const router = useRouter();
  const { i18n } = useTranslation();
  const { login, busy, status } = useSession();
  const t = i18n.getFixedT(i18n.language, 'guest') as unknown as (key: string) => string;
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [error, setError] = useState<string | null>(null);

  const submit = async (event: React.FormEvent) => {
    event.preventDefault();
    setError(null);
    const code = await login(email.trim(), password);
    if (!code) {
      router.push('/account');
      return;
    }
    setError(code === 'INVALID_CREDENTIALS' || code === 'ACCOUNT_BLOCKED' ? t('invalidCredentials') : t('errorGeneric'));
  };

  return {
    email,
    setEmail,
    password,
    setPassword,
    submit,
    error,
    busy,
    authenticated: status === 'authenticated',
    labels: {
      title: t('signInTitle'),
      subtitle: t('signInSubtitle'),
      email: t('email'),
      password: t('password'),
      submit: t('signIn'),
      noAccount: t('noAccount'),
      register: t('register'),
    },
  };
};
