import { useEffect, useMemo, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { useDebouncedValue } from '@mungini/ui-web';
import { adminApi, type AdminHostView, type HostReferenceView } from '@/services/admin/admin.api';

const PAGE_SIZE = 20;

export const useHosts = () => {
  const { t } = useTranslation('admin');
  const translate = t as unknown as (key: string) => string;
  const { client } = useSession();
  const api = useMemo(() => adminApi(client), [client]);
  const [status, setStatus] = useState('');
  const [search, setSearch] = useState('');
  const term = useDebouncedValue(search, 250);
  const [page, setPage] = useState(1);
  const [items, setItems] = useState<AdminHostView[]>([]);
  const [total, setTotal] = useState(0);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [selected, setSelected] = useState<AdminHostView | null>(null);
  const [reference, setReference] = useState<HostReferenceView | null>(null);
  const [action, setAction] = useState('');
  const [reason, setReason] = useState('');
  const [busy, setBusy] = useState(false);

  useEffect(() => {
    setPage(1);
  }, [status, term]);

  useEffect(() => {
    let active = true;
    setLoading(true);
    setError(null);
    void api.hosts(status, term, page, PAGE_SIZE).then((result) => {
      if (!active) return;
      setLoading(false);
      if (result.error) {
        setError(t('error'));
        return;
      }
      setItems(result.data?.items ?? []);
      setTotal(result.data?.total ?? 0);
    });
    return () => {
      active = false;
    };
  }, [api, status, term, page, t]);

  useEffect(() => {
    void api.reference().then((result) => {
      if (result.data) setReference(result.data);
    });
  }, [api]);

  const open = (row: AdminHostView) => {
    setSelected(row);
    setAction('');
    setReason('');
  };

  const close = () => setSelected(null);

  const reasons = action === 'SUSPEND' ? reference?.suspendReasons ?? [] : action === 'REINSTATE' ? reference?.reinstateReasons ?? [] : [];

  const apply = async () => {
    if (!selected || !action || !reason) return;
    setBusy(true);
    const result = await api.setHostStatus(selected.id, action, reason);
    setBusy(false);
    if (result.error) {
      setError(t('error'));
      return;
    }
    setItems((current) => current.map((host) => (host.id === selected.id ? { ...host, status: result.data?.status ?? host.status } : host)));
    setSelected(null);
  };

  return {
    status,
    setStatus,
    search,
    setSearch,
    page,
    setPage,
    pageSize: PAGE_SIZE,
    items,
    total,
    loading,
    error,
    selected,
    open,
    close,
    action,
    setAction,
    reason,
    setReason,
    reasons,
    apply,
    busy,
    labels: {
      title: t('hostsTitle'),
      subtitle: t('hostsSubtitle'),
      host: t('hostName'),
      city: t('city'),
      members: t('members'),
      properties: t('properties'),
      status: t('status'),
      search: t('search'),
      all: t('allStatuses'),
      suspend: t('suspend'),
      reinstate: t('reinstate'),
      reason: t('reason'),
      chooseReason: t('chooseReason'),
      confirm: t('confirm'),
      cancel: t('cancel'),
      error: t('error'),
      empty: t('hostsEmpty'),
      statusLabel: (value: string) => translate(`hostStatuses.${value}`),
      reasonLabel: (value: string) => translate(`reasonLabels.${value}`),
    },
  };
};
