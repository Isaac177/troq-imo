import { useEffect, useState } from 'react';
import type { components } from '@mungini/api';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { formatDateTime } from '@mungini/utils';
import { apiBaseUrl } from '@/env';
import { createFeed, listFeeds, revokeFeed } from '@/services/calendar/feeds.api';

const PAGE_SIZE = 10;
type Feed = components['schemas']['FeedTokenView'];

export const useCalendarFeeds = (propertyId: string, timezone: string) => {
  const { t } = useTranslation('hostCalendar');
  const translate = t as unknown as (key: string) => string;
  const { client } = useSession();
  const [open, setOpen] = useState(false);
  const [items, setItems] = useState<Feed[]>([]);
  const [page, setPage] = useState(1);
  const [total, setTotal] = useState(0);
  const [revision, setRevision] = useState(0);
  const [link, setLink] = useState('');
  const [target, setTarget] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');
  const [copied, setCopied] = useState(false);
  const pages = Math.max(1, Math.ceil(total / PAGE_SIZE));

  useEffect(() => {
    if (!open) return;
    let active = true;
    setLoading(true);
    setError('');
    void listFeeds(client, propertyId, page).then((result) => {
      if (!active) return;
      if (!result.data || result.error) setError(translate('feedError'));
      else { setItems(result.data.items); setTotal(result.data.total); }
    }).catch(() => { if (active) setError(translate('feedError')); }).finally(() => { if (active) setLoading(false); });
    return () => { active = false; };
  }, [client, propertyId, page, revision, open, translate]);

  const create = async () => {
    setBusy(true); setError(''); setCopied(false); setLink('');
    try {
      const result = await createFeed(client, propertyId);
      if (!result.data || result.error) setError(translate('feedError'));
      else { setLink(new URL(result.data.feedPath, new URL(apiBaseUrl || '/', window.location.origin)).toString()); setPage(1); setRevision((value) => value + 1); }
    } catch { setError(translate('feedError')); }
    finally { setBusy(false); }
  };
  const revoke = async () => {
    if (!target) return;
    setBusy(true); setError('');
    try {
      const result = await revokeFeed(client, target);
      if (!result.data || result.error) setError(translate('feedError'));
      else { setTarget(null); setLink(''); setPage(1); setRevision((value) => value + 1); }
    } catch { setError(translate('feedError')); }
    finally { setBusy(false); }
  };
  const copy = async () => {
    try { await navigator.clipboard.writeText(link); setCopied(true); }
    catch { setCopied(false); setError(translate('feedCopyError')); }
  };
  return {
    open, toggle: () => setOpen((value) => !value), items: items.map((item) => ({ id: item.id, created: formatDateTime(item.createdAt, timezone) })),
    page, pages, previous: () => setPage((value) => Math.max(1, value - 1)), next: () => setPage((value) => Math.min(pages, value + 1)),
    link, busy, loading, error, copied, create, copy, target, setTarget, revoke,
    labels: {
      title: translate('feedTitle'), description: translate('feedDescription'), create: translate('feedCreate'), once: translate('feedOnce'),
      url: translate('feedUrl'), copy: translate('feedCopy'), copied: translate('feedCopied'), empty: translate('feedEmpty'), created: translate('feedCreated'),
      revoke: translate('feedRevoke'), confirm: translate('feedConfirm'), consequence: translate('feedConsequence'), cancel: translate('feedCancel'),
      previous: translate('previous'), next: translate('next'), loading: translate('loading'),
    },
  };
};
