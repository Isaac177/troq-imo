import { useEffect, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { getPreferences, listCities, setPreference, updateProfile } from '@/services/settings/settings.api';
import type { Preference } from '@/types/settings/settings';

type Channel = 'inApp' | 'email' | 'push';

export const useHostSettings = () => {
  const { t } = useTranslation('hostSettings');
  const translate = t as unknown as (key: string) => string;
  const { session, client, city, reload } = useSession();
  const [preferences, setPreferences] = useState<Preference[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [notice, setNotice] = useState<string | null>(null);
  const [savingKey, setSavingKey] = useState<string | null>(null);
  const [cityName, setCityName] = useState(city);
  const [editOpen, setEditOpen] = useState(false);
  const [firstName, setFirstName] = useState('');
  const [lastName, setLastName] = useState('');
  const [preferredLanguage, setPreferredLanguage] = useState('en');
  const [accountBusy, setAccountBusy] = useState(false);
  const [accountError, setAccountError] = useState<string | null>(null);

  useEffect(() => {
    let active = true;
    setLoading(true);
    setError(null);
    void getPreferences(client).then((result) => {
      if (!active) return;
      setLoading(false);
      if (result.error || !result.data) {
        setError(translate('error'));
        setPreferences([]);
        return;
      }
      setPreferences(result.data.items);
    });
    return () => {
      active = false;
    };
  }, [client, translate]);

  useEffect(() => {
    let active = true;
    void listCities(client).then(({ data }) => {
      const match = data?.data.find((item) => item.code === city);
      if (active && match) setCityName(match.name);
    }).catch(() => undefined);
    return () => {
      active = false;
    };
  }, [client, city]);

  const toggle = async (preference: Preference, channel: Channel, checked: boolean) => {
    const next: Preference = { ...preference, [channel]: checked };
    setPreferences((current) => current.map((item) => (item.eventKey === preference.eventKey ? next : item)));
    setSavingKey(`${preference.eventKey}.${channel}`);
    setError(null);
    setNotice(null);
    const result = await setPreference(client, next);
    setSavingKey(null);
    if (result.error) {
      setPreferences((current) => current.map((item) => (item.eventKey === preference.eventKey ? preference : item)));
      setError(translate('error'));
      return;
    }
    setNotice(translate('saved'));
  };

  const eventLabel = (key: string) => {
    const label = translate(`eventLabels.${key}`);
    return label === `eventLabels.${key}` ? translate('generalUpdate') : label;
  };

  const openAccount = () => {
    setFirstName(user?.firstName ?? '');
    setLastName(user?.lastName ?? '');
    setPreferredLanguage(user?.preferredLanguage ?? 'en');
    setAccountError(null);
    setEditOpen(true);
  };

  const saveAccount = async () => {
    setAccountBusy(true);
    setAccountError(null);
    const result = await updateProfile(client, { firstName, lastName, preferredLanguage });
    setAccountBusy(false);
    if (result.error) {
      setAccountError(translate('error'));
      return;
    }
    await reload();
    setEditOpen(false);
    setNotice(translate('accountSaved'));
  };

  const user = session?.user;
  return {
    preferences,
    loading,
    error,
    notice,
    savingKey,
    eventLabel,
    toggle,
    account: {
      name: [user?.firstName, user?.lastName].filter(Boolean).join(' ') || '—',
      email: user?.email || '—',
      phone: user?.phone || '—',
      city: cityName,
    },
    editOpen, setEditOpen, openAccount, firstName, setFirstName, lastName, setLastName, preferredLanguage, setPreferredLanguage, accountBusy, accountError, saveAccount,
    languageOptions: [
      { value: 'en', label: translate('languages.en') },
      { value: 'fr', label: translate('languages.fr') },
    ],
    labels: {
      title: translate('title'),
      subtitle: translate('subtitle'),
      account: translate('account'),
      editAccount: translate('editAccount'),
      firstName: translate('firstName'),
      lastName: translate('lastName'),
      preferredLanguage: translate('preferredLanguage'),
      accountSaved: translate('accountSaved'),
      save: translate('save'),
      close: translate('close'),
      name: translate('name'),
      email: translate('email'),
      phone: translate('phone'),
      city: translate('city'),
      notifications: translate('notifications'),
      notificationsHint: translate('notificationsHint'),
      inApp: translate('inApp'),
      emailChannel: translate('email'),
      push: translate('push'),
      loading: translate('loading'),
      error: translate('error'),
      saved: translate('saved'),
    },
  };
};
