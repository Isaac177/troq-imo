import { useCallback, useEffect, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { acceptInvitation, listInvitations } from '@/services/account/invitations.api';
import type { InvitationPage } from '@/types/account/invitations';

export const useInvitations = () => {
  const { client, status, city, reload } = useSession();
  const { i18n } = useTranslation();
  const t = i18n.getFixedT(i18n.language, 'guest') as unknown as (key: string) => string;
  const host = i18n.getFixedT(i18n.language, 'host') as unknown as (key: string) => string;
  const [page, setPage] = useState(1);
  const [data, setData] = useState<InvitationPage | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(false);
  const [cityError, setCityError] = useState(false);
  const [accepting, setAccepting] = useState<string | null>(null);
  const [refresh, setRefresh] = useState(0);
  const [accepted, setAccepted] = useState(false);

  useEffect(() => { setPage(1); }, [city]);
  useEffect(() => {
    if (status !== 'authenticated') return;
    let active = true;
    setLoading(true);
    setError(false);
    void listInvitations(client, page).then((result) => {
      if (!active) return;
      if (!result.items.length && result.total > 0 && page > 1) { setPage(page - 1); return; }
      setData(result);
    }).catch(() => { if (active) setError(true); }).finally(() => { if (active) setLoading(false); });
    return () => { active = false; };
  }, [client, status, city, page, refresh]);

  const accept = useCallback(async (id: string) => {
    setAccepting(id);
    setError(false);
    setAccepted(false);
    setCityError(false);
    try {
      await acceptInvitation(client, id);
      setAccepted(true);
      void reload().catch(() => undefined);
      setRefresh((value) => value + 1);
    } catch (failure) { setCityError(failure instanceof Error && (failure.message === 'CITY_MISMATCH' || failure.message === 'CITY_VIEW_ONLY')); setError(true); }
    finally { setAccepting(null); }
  }, [client, reload]);

  const formatDate = (value: string) => {
    const date = new Date(value);
    if (Number.isNaN(date.getTime())) return '';
    return new Intl.DateTimeFormat('en-GB', { day: '2-digit', month: '2-digit', year: 'numeric', timeZone: 'UTC' }).format(date);
  };

  return {
    data, loading, error, accepting, accepted, page, accept,
    previous: () => setPage((value) => Math.max(1, value - 1)),
    next: () => setPage((value) => value + 1),
    retry: () => setRefresh((value) => value + 1),
    formatDate,
    permission: (value: string) => i18n.exists(`permissionLabels.${value}`, { ns: 'host' }) ? host(`permissionLabels.${value}`) : t('invitationOtherPermission'),
    labels: {
      title: t('invitationsTitle'), empty: t('invitationsEmpty'), loading: t('invitationsLoading'),
      expires: t('invitationExpires'), permissions: t('invitationPermissions'), accept: t('invitationAccept'),
      error: t(cityError ? 'invitationCityError' : 'invitationError'), accepted: t('invitationAccepted'), retry: t('invitationRetry'),
      previous: t('invitationPrevious'), next: t('invitationNext'), page: t('invitationPage'),
    },
  };
};
