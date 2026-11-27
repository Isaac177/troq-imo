import { useCallback, useEffect, useState } from 'react';
import { useSearchParams } from 'react-router-dom';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { useDebouncedValue } from '@mungini/ui-web';
import { formatDateTime } from '@mungini/utils';
import { listConversations } from '@/services/messages/messages.api';
import type { Conversation } from '@/types/messages/messages';

export const useHostMessages = () => {
  const { t, i18n } = useTranslation('hostMessages');
  const translate = t as unknown as (key: string) => string;
  const { client, session, can } = useSession();
  const hosts = (session?.hosts ?? []).filter((host) => can('messages.write', host.hostId));
  const [params, setParams] = useSearchParams();
  const selectedHost = params.get('host') ?? '';
  const hostId = hosts.some((host) => host.hostId === selectedHost) ? selectedHost : hosts[0]?.hostId ?? '';
  const selected = params.get('reservation') ?? '';
  const [search, setSearch] = useState(params.get('q') ?? '');
  const term = useDebouncedValue(search, 250);
  const pageValue = Number(params.get('page') ?? '1');
  const page = Number.isSafeInteger(pageValue) && pageValue > 0 ? pageValue : 1;
  const [rows, setRows] = useState<Conversation[]>([]);
  const [total, setTotal] = useState(0);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');
  const [readRevision, setReadRevision] = useState(0);
  const onRead = useCallback(() => setReadRevision((value) => value + 1), []);
  const update = (values: Record<string, string>) => {
    const next = new URLSearchParams(params);
    Object.entries(values).forEach(([key, value]) => value ? next.set(key, value) : next.delete(key));
    setParams(next, { replace: true });
  };
  useEffect(() => {
    if (term !== (params.get('q') ?? '')) update({ q: term, page: '' });
  }, [term]);
  useEffect(() => {
    if (!hostId) return;
    let active = true;
    setLoading(true);
    setError('');
    void listConversations(client, hostId, page, params.get('q') ?? '').then((result) => {
      if (!active) return;
      setLoading(false);
      if (result.error || !result.data) { setRows([]); setTotal(0); setError(translate('error')); return; }
      setRows(result.data.items);
      setTotal(result.data.total);
    }).catch(() => { if (active) { setLoading(false); setError(translate('error')); } });
    return () => { active = false; };
  }, [client, hostId, page, params, translate, readRevision]);
  return {
    hosts: hosts.map((host) => ({ value: host.hostId, label: host.displayName })),
    hostId, setHostId: (id: string) => update({ host: id, page: '', reservation: '' }),
    search, setSearch, page, total, loading, error, selected, rows, onRead,
    setSelected: (id: string) => update({ reservation: id }),
    previous: () => update({ page: String(Math.max(1, page - 1)) }),
    next: () => update({ page: String(page + 1) }),
    canPrevious: page > 1, canNext: page * 20 < total,
    name: (row: Conversation) => (i18n.language.startsWith('fr') ? row.propertyNameFr : row.propertyNameEn) || row.propertyNameEn || row.propertyNameFr,
    time: (row: Conversation) => formatDateTime(row.lastAt, row.timezone),
    unread: (count: number) => translate('unread').replace('{{count}}', String(count)),
    labels: { title: translate('title'), subtitle: translate('subtitle'), search: translate('search'), host: translate('host'), empty: translate('empty'), choose: translate('choose'), previous: translate('previous'), next: translate('next') },
  };
};
