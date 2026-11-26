import { useEffect, useState } from 'react';
import { useRouter } from 'next/navigation';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { formatDateTime } from '@mungini/utils';
import { getUnreadCount, listNotificationCities, listNotifications, markAllNotificationsRead, markNotificationRead } from '@/services/notifications/notifications.api';
import type { Notification, NotificationCity, NotificationFilter, NotificationPage, NotificationRowView } from '@/types/notifications/notifications';

const typeKeys: Record<string, string> = {
  MESSAGE: 'types.message',
  TICKET_UPDATE: 'types.ticketUpdate',
  TICKET_ASSIGNED: 'types.ticketAssigned',
  TICKET_COMMENT: 'types.ticketComment',
  TICKET_RESOLVED: 'types.ticketResolved',
  SYSTEM_UPDATE: 'types.systemUpdate',
  SECURITY_ALERT: 'types.securityAlert',
  PAYMENT_UPDATE: 'types.paymentUpdate',
  ORDER_UPDATE: 'types.orderUpdate',
  REVIEW_RECEIVED: 'types.reviewReceived',
  ANNOUNCEMENT: 'types.announcement',
  REMINDER: 'types.reminder',
  PROMOTION: 'types.promotion',
  INFO: 'types.info',
  ALERT: 'types.alert',
};

const priorityKeys: Record<string, string> = {
  LOW: 'priorities.low',
  NORMAL: 'priorities.normal',
  HIGH: 'priorities.high',
  URGENT: 'priorities.urgent',
};

const isInternalLink = (link: string | null | undefined): link is string => Boolean(link && link.startsWith('/') && !link.startsWith('//'));

const readSearchState = () => {
  const params = new URLSearchParams(window.location.search);
  const requested = Number(params.get('page'));
  const filter: NotificationFilter = params.get('filter') === 'unread' ? 'unread' : 'all';
  return { filter, page: Number.isInteger(requested) && requested > 0 ? requested : 1 };
};

export const useNotifications = () => {
  const { client, status, city } = useSession();
  const { t: raw, i18n } = useTranslation('guestNotifications');
  const router = useRouter();
  const t = raw as unknown as (key: string) => string;
  const [cities, setCities] = useState<NotificationCity[]>([]);
  const [data, setData] = useState<NotificationPage | null>(null);
  const [page, setPage] = useState(1);
  const [filter, setFilter] = useState<NotificationFilter>('all');
  const [unread, setUnread] = useState<number | null>(null);
  const [loading, setLoading] = useState(false);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState(false);
  const [actionError, setActionError] = useState('');
  const [refresh, setRefresh] = useState(0);
  const [urlReady, setUrlReady] = useState(false);

  useEffect(() => {
    const search = readSearchState();
    setFilter(search.filter);
    setPage(search.page);
    setUrlReady(true);
  }, []);

  useEffect(() => {
    if (!urlReady) return;
    const params = new URLSearchParams();
    if (filter === 'unread') params.set('filter', 'unread');
    if (page > 1) params.set('page', String(page));
    const query = params.toString();
    window.history.replaceState(null, '', `${window.location.pathname}${query ? `?${query}` : ''}`);
  }, [filter, page, urlReady]);

  useEffect(() => {
    let active = true;
    void listNotificationCities(client).then((items) => {
      if (active) setCities(items);
    }).catch(() => undefined);
    return () => { active = false; };
  }, [client]);

  useEffect(() => {
    if (status !== 'authenticated') return;
    let active = true;
    void getUnreadCount(client).then((value) => {
      if (active) setUnread(value);
    }).catch(() => undefined);
    return () => { active = false; };
  }, [client, status, refresh]);

  useEffect(() => {
    if (status !== 'authenticated' || !urlReady) return;
    let active = true;
    setLoading(true);
    setError(false);
    void listNotifications(client, page, filter === 'unread', i18n.language).then((result) => {
      if (active) setData(result);
    }).catch(() => {
      if (active) {
        setData(null);
        setError(true);
      }
    }).finally(() => {
      if (active) setLoading(false);
    });
    return () => { active = false; };
  }, [client, status, page, filter, i18n.language, refresh, urlReady]);

  useEffect(() => {
    if (loading || error || !data || page <= 1) return;
    if (data.items.length === 0 && data.total > 0) setPage(page - 1);
  }, [loading, error, data, page]);

  const applyRead = (item: Notification) => {
    setData((current) => {
      if (!current) return current;
      if (filter === 'unread') {
        return { ...current, items: current.items.filter((row) => row.id !== item.id), total: Math.max(0, current.total - 1) };
      }
      return { ...current, items: current.items.map((row) => (row.id === item.id ? { ...row, read: true, readAt: row.readAt ?? new Date().toISOString() } : row)) };
    });
    setUnread((value) => (value === null ? null : Math.max(0, value - 1)));
  };

  const markRead = (item: Notification) => {
    if (item.read) return;
    setActionError('');
    applyRead(item);
    void markNotificationRead(client, item.id).catch(() => {
      setActionError(t('actionError'));
      setRefresh((value) => value + 1);
    });
  };

  const open = (item: Notification) => {
    markRead(item);
    if (isInternalLink(item.deepLink)) router.push(item.deepLink);
  };

  const markAllRead = async () => {
    if (busy || !unread) return;
    setBusy(true);
    setActionError('');
    try {
      await markAllNotificationsRead(client);
      setUnread(0);
      setData((current) => {
        if (!current) return current;
        if (filter === 'unread') return { ...current, items: [], total: 0 };
        return { ...current, items: current.items.map((row) => ({ ...row, read: true, readAt: row.readAt ?? new Date().toISOString() })) };
      });
      if (filter === 'unread' && page > 1) setPage(1);
    } catch {
      setActionError(t('actionError'));
      setRefresh((value) => value + 1);
    } finally {
      setBusy(false);
    }
  };

  const timezone = cities.find((item) => item.code === city)?.timezone ?? 'UTC';
  const describe = (item: Notification): NotificationRowView => {
    const navigable = isInternalLink(item.deepLink);
    return {
      id: item.id,
      title: item.title,
      body: item.body,
      read: item.read,
      readLabel: item.read ? t('readState') : t('unreadState'),
      tone: item.read ? 'muted' : 'primary',
      typeLabel: t(typeKeys[item.type] ?? 'types.unknown'),
      priorityLabel: t(priorityKeys[item.priority] ?? 'priorities.normal'),
      timestamp: formatDateTime(item.createdAt, timezone),
      navigable,
      interactive: !item.read || navigable,
      open: () => open(item),
    };
  };

  const changeFilter = (value: string) => {
    setFilter(value === 'unread' ? 'unread' : 'all');
    setPage(1);
    setActionError('');
  };

  return {
    status,
    data,
    filter,
    page,
    unread,
    loading,
    busy,
    error,
    actionError,
    items: data?.items ?? [],
    unreadSummary: unread === null ? '' : unread > 0 ? t('unreadSummary').replace('{count}', String(unread)) : t('allRead'),
    tabs: [
      { value: 'all', label: t('all') },
      { value: 'unread', label: t('unread'), count: unread ?? undefined },
    ],
    labels: {
      title: t('title'),
      signInPrompt: t('signInPrompt'),
      signIn: t('signIn'),
      filter: t('filter'),
      markAllRead: t('markAllRead'),
      loading: t('loading'),
      emptyAll: t('emptyAll'),
      emptyAllHint: t('emptyAllHint'),
      emptyUnread: t('emptyUnread'),
      emptyUnreadHint: t('emptyUnreadHint'),
      explore: t('explore'),
      error: t('error'),
      retry: t('retry'),
    },
    describe,
    changeFilter,
    changePage: (value: number) => {
      setPage(value);
      setActionError('');
    },
    markAllRead: () => { void markAllRead(); },
    retry: () => setRefresh((value) => value + 1),
  };
};
