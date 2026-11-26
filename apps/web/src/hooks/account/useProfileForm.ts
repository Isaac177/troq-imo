import { useEffect, useState } from 'react';
import type { ChangeEvent, FormEvent } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { updateProfile } from '@/services/account/profile.api';

export const useProfileForm = () => {
  const { i18n } = useTranslation();
  const t = i18n.getFixedT(i18n.language, 'guest') as unknown as (key: string) => string;
  const { client, session, reload } = useSession();
  const [firstName, setFirstName] = useState('');
  const [lastName, setLastName] = useState('');
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [saved, setSaved] = useState(false);

  useEffect(() => {
    setFirstName(session?.user.firstName ?? '');
    setLastName(session?.user.lastName ?? '');
  }, [session?.user.id, session?.user.firstName, session?.user.lastName]);

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
      await updateProfile(client, { firstName: first || undefined, lastName: last || undefined });
      await reload();
      setSaved(true);
    } catch {
      setError(t('profileSaveError'));
    } finally {
      setBusy(false);
    }
  };

  return {
    firstName, lastName, busy, error, saved, save,
    changeFirstName: (event: ChangeEvent<HTMLInputElement>) => setFirstName(event.target.value),
    changeLastName: (event: ChangeEvent<HTMLInputElement>) => setLastName(event.target.value),
    labels: {
      title: t('profile'), firstName: t('firstName'), lastName: t('lastName'),
      save: t('profileSave'), saving: t('profileSaving'), saved: t('profileSaved'),
    },
  };
};
