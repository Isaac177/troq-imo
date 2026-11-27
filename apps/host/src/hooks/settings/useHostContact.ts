import { useEffect, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { getHostContact, updateHostContact } from '@/services/settings/contact.api';

export const useHostContact = () => {
  const { t } = useTranslation('hostSettings');
  const translate = t as unknown as (key: string) => string;
  const { client, session, can } = useSession();
  const hosts = (session?.hosts ?? []).filter((host) => can('properties.read', host.hostId) || can('properties.write', host.hostId));
  const [hostId, setHostId] = useState('');
  const [phone, setPhone] = useState('');
  const [email, setEmail] = useState('');
  const [loadedHostId, setLoadedHostId] = useState('');
  const [loading, setLoading] = useState(false);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState('');
  const [notice, setNotice] = useState('');

  useEffect(() => {
    if (!hosts.some((host) => host.hostId === hostId)) setHostId(hosts[0]?.hostId ?? '');
  }, [session, can, hostId]);

  useEffect(() => {
    if (!hostId) return;
    let active = true;
    setLoading(true);
    setLoadedHostId('');
    setError('');
    setNotice('');
    void getHostContact(client, hostId).then((result) => {
      if (!active) return;
      setLoading(false);
      if (result.error || !result.data) {
        setError(translate('contactLoadError'));
        return;
      }
      setPhone(result.data.guestContactPhone);
      setEmail(result.data.guestContactEmail);
      setLoadedHostId(hostId);
    }).catch(() => {
      if (active) { setLoading(false); setError(translate('contactLoadError')); }
    });
    return () => { active = false; };
  }, [client, hostId, translate]);

  const save = async () => {
    if (!hostId || loadedHostId !== hostId || saving || !can('properties.write', hostId)) return;
    setSaving(true);
    setError('');
    setNotice('');
    try {
      const result = await updateHostContact(client, hostId, { guestContactPhone: phone, guestContactEmail: email });
      if (result.error || !result.data) { setError(translate('contactSaveError')); return; }
      setPhone(result.data.guestContactPhone);
      setEmail(result.data.guestContactEmail);
      setNotice(translate('contactSaved'));
    } catch {
      setError(translate('contactSaveError'));
    } finally {
      setSaving(false);
    }
  };

  return {
    available: hosts.length > 0,
    hosts: hosts.map((host) => ({ value: host.hostId, label: host.displayName })),
    hostId, setHostId, phone, setPhone, email, setEmail, loading, saving, error, notice, save,
    disabled: loading || saving || loadedHostId !== hostId || !can('properties.write', hostId),
    labels: {
      title: translate('contactTitle'), hint: translate('contactHint'), host: translate('contactHost'),
      email: translate('contactEmail'), phone: translate('contactPhone'), save: translate('save'), loading: translate('loading'),
    },
  };
};
