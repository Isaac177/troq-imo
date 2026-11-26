import { useEffect, useState } from 'react';
import { useRouter } from 'next/navigation';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { apiErrorCode } from '@mungini/session';

export const useRegisterView = () => {
  const router = useRouter();
  const { i18n } = useTranslation();
  const { client, register, status, busy } = useSession();
  const t = i18n.getFixedT(i18n.language, 'guest') as unknown as (key: string) => string;
  const [step, setStep] = useState(1);
  const [email, setEmail] = useState('');
  const [code, setCode] = useState('');
  const [proof, setProof] = useState('');
  const [password, setPassword] = useState('');
  const [firstName, setFirstName] = useState('');
  const [lastName, setLastName] = useState('');
  const [city, setCity] = useState('GOMA');
  const [cities, setCities] = useState<{ code: string; name: string }[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [sending, setSending] = useState(false);

  useEffect(() => {
    void client.api.GET('/api/public/cities').then(({ data }) => {
      if (data) setCities(data.data);
    });
  }, [client]);

  const message = (value: string) => {
    if (value === 'REGISTRATION_CONTACT_EXISTS') return t('contactExists');
    if (value === 'REGISTRATION_OTP_INVALID') return t('otpInvalid');
    if (value === 'REGISTRATION_CONTACT_INVALID') return t('contactInvalid');
    return t('errorGeneric');
  };

  const sendCode = async (event: React.FormEvent) => {
    event.preventDefault();
    setError(null);
    setSending(true);
    const { error: responseError } = await client.api.POST('/api/public/auth/register/otp', {
      body: { channel: 'email', contact: email.trim(), language: i18n.language },
    });
    setSending(false);
    if (responseError) {
      setError(message(apiErrorCode(responseError)));
      return;
    }
    setStep(2);
  };

  const verify = async (event: React.FormEvent) => {
    event.preventDefault();
    setError(null);
    setSending(true);
    const { data, error: responseError } = await client.api.POST('/api/public/auth/register/verify', {
      body: { channel: 'email', contact: email.trim(), code: code.trim() },
    });
    setSending(false);
    if (responseError || !data) {
      setError(message(apiErrorCode(responseError)));
      return;
    }
    setProof(data.data.proof);
    setStep(3);
  };

  const finish = async (event: React.FormEvent) => {
    event.preventDefault();
    setError(null);
    if (!password.trim()) {
      setError(t('passwordRequired'));
      return;
    }
    const result = await register({
      channel: 'email',
      contact: email.trim(),
      proof,
      password,
      firstName: firstName.trim() || undefined,
      lastName: lastName.trim() || undefined,
      cityCode: city,
      preferredLanguage: i18n.language,
    });
    if (result) {
      setError(message(result));
      return;
    }
    router.push('/account');
  };

  return {
    step,
    setStep,
    email,
    setEmail,
    code,
    setCode,
    password,
    setPassword,
    firstName,
    setFirstName,
    lastName,
    setLastName,
    city,
    setCity,
    cities,
    error,
    sending: sending || busy,
    sendCode,
    verify,
    finish,
    authenticated: status === 'authenticated',
    labels: {
      title: t('registerTitle'),
      subtitle: t('registerSubtitle'),
      email: t('email'),
      code: t('code'),
      sendCode: t('sendCode'),
      verify: t('verify'),
      password: t('password'),
      firstName: t('firstName'),
      lastName: t('lastName'),
      city: t('city'),
      create: t('createAccount'),
      haveAccount: t('haveAccount'),
      signIn: t('signIn'),
      stepContact: t('steps.contact'),
      stepCode: t('steps.code'),
      stepDetails: t('steps.details'),
    },
  };
};
