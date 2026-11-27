import { useEffect, useState } from 'react';
import type { ChangeEvent, FormEvent } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { useRouter } from 'next/navigation';
import { getLanguages, updateProfile } from '@/services/account/profile.api';

export const useProfileForm = () => {
  const { i18n } = useTranslation();
  const router = useRouter();
  const t = i18n.getFixedT(i18n.language, 'guest') as unknown as (key: string) => string;
  const { client, session, reload } = useSession();
  const [firstName, setFirstName] = useState('');
  const [lastName, setLastName] = useState('');
  const [language, setLanguage] = useState('');
  const [languages, setLanguages] = useState<string[]>([]);
  const [languageError, setLanguageError] = useState(false);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [saved, setSaved] = useState(false);

  useEffect(() => {
    setFirstName(session?.user.firstName ?? '');
    setLastName(session?.user.lastName ?? '');
    setLanguage(session?.user.preferredLanguage ?? '');
  }, [session?.user.id, session?.user.firstName, session?.user.lastName, session?.user.preferredLanguage]);

  useEffect(() => {
    let active = true;
    void getLanguages(client).then((items) => {
      if (active) setLanguages(items);
    }).catch(() => {
      if (active) setLanguageError(true);
    });
    return () => { active = false; };
  }, [client]);

  const save = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setError(null);
    setSaved(false);
    const first = firstName.trim();
    const last = lastName.trim();
    if ((!first && session?.user.firstName) || (!last && session?.user.lastName)) {
      setError(t('profileNameRequired'));
      return;
    }
    if ([...first].length > 80 || [...last].length > 80) {
      setError(t('profileNameTooLong'));
      return;
    }
    setBusy(true);
    try {
      const preferredLanguage = languages.includes(language) ? language : undefined;
      await updateProfile(client, { firstName: first || undefined, lastName: last || undefined, preferredLanguage });
      await reload();
      if (preferredLanguage) {
        await i18n.changeLanguage(preferredLanguage);
        document.documentElement.lang = preferredLanguage;
        document.cookie = `mungini-language=${preferredLanguage}; path=/; max-age=31536000; samesite=lax`;
        router.refresh();
      }
      setSaved(true);
    } catch {
      setError(t('profileSaveError'));
    } finally {
      setBusy(false);
    }
  };

  return {
    firstName, lastName, language, setLanguage, busy, error, saved, save, languageError,
    languageOptions: languages.map((code) => ({ value: code, label: t(`languages.${code}`) })),
    changeFirstName: (event: ChangeEvent<HTMLInputElement>) => setFirstName(event.target.value),
    changeLastName: (event: ChangeEvent<HTMLInputElement>) => setLastName(event.target.value),
    labels: {
      title: t('profile'), firstName: t('firstName'), lastName: t('lastName'),
      save: t('profileSave'), saving: t('profileSaving'), saved: t('profileSaved'),
      language: t('language'), languageError: t('profileLanguagesError'),
    },
  };
};
