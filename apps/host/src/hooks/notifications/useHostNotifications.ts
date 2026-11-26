import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import type { StatusTone } from '@mungini/theme/tones';
import { formatDateTime } from '@mungini/utils';
import { getUnreadCount, listCities, listNotifications, markAllNotificationsRead, markNotificationRead } from '@/services/notifications/notifications.api';
import type { HostNotification, NotificationFilter, NotificationRowModel } from '@/types/notifications/notifications';

const PAGE_SIZE = 20;
const KNOWN_ERRORS = ['NOTIFICATION_NOT_FOUND', 'FORBIDDEN'];

const isInternalLink = (link: string | null | undefined): link is string => Boolean(link && link.startsWith('/') && !link.startsWith('//'));

const priorityTone = (priority: string): StatusTone => {
  if (priority === 'URGENT') return 'danger';
  if (priority === 'HIGH') return 'warning';
  if (priority === 'NORMAL') return 'neutral';
  return 'muted';
};

export const useHostNotifications = () => {
  const { t, i18n } = useTranslation('hostNotifications');
  const translate = t as unknown as (key: string, options?: Record<string, unknown>) => string;
  const { client, city } = useSession();
  const navigate = useNavigate();
  const [filter, setFilter] = useState<NotificationFilter>('all');
  const [page, setPage] = useState(1);
  const [rows, setRows] = useState<HostNotification[]>([]);
  const [total, setTotal] = useState(0);
  const [unreadCount, setUnreadCount] = useState(0);
  const [loading, setLoading] = useState(true);
  const [markingAll, setMarkingAll] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [notice, setNotice] = useState<string | null>(null);
  const [refreshKey, setRefreshKey] = useState(0);
  const [timezone, setTimezone] = useState('UTC');
  const locale = i18n.language.startsWith('fr') ? 'fr' : 'en';

  const message = (code: string | null) => (code && KNOWN_ERRORS.includes(code) ? translate(`errors.${code}`) : translate('errors.generic'));
  const label = (group: 'types' | 'priorities', value: string) => translate(`${group}.${value}`, { defaultValue: value });

  useEffect(() => {
    let active = true;
    void listCities(client).then(({ data }) => {
      const match = data?.data.find((item) => item.code === city);
      if (active && match) setTimezone(match.timezone);
    }).catch(() => undefined);
    return () => {
      active = false;
    };
  }, [client, city]);

  useEffect(() => {
    let active = true;
    void getUnreadCount(client).then((result) => {
      if (active && result.data) setUnreadCount(result.data.unread);
    }).catch(() => undefined);
    return () => {
      active = false;
    };
  }, [client, refreshKey]);

  useEffect(() => {
    let active = true;
    setLoading(true);
    setError(null);
    void listNotifications(client, { locale, filter, page, pageSize: PAGE_SIZE }).then((result) => {
      if (!active) return;
      setLoading(false);
      if (result.error || !result.data) {
        setError(translate('error'));
        setRows([]);
        setTotal(0);
        return;
      }
      setRows(result.data.items);
      setTotal(result.data.total);
    }).catch(() => {
      if (!active) return;
      setLoading(false);
      setError(translate('error'));
      setRows([]);
      setTotal(0);
    });
    return () => {
      active = false;
    };
  }, [client, locale, filter, page, refreshKey, translate]);

  useEffect(() => {
    if (!loading && !error && rows.length === 0 && page > 1 && total > 0) setPage(page - 1);
  }, [loading, error, rows.length, page, total]);

  const markRead = async (item: HostNotification) => {
    setError(null);
    try {
      const result = await markNotificationRead(client, item.id);
      if (result.error || !result.data) {
        setError(message(result.error));
        return;
      }
    } catch {
      setError(message(null));
      return;
    }
    if (filter === 'unread') {
      setRows((current) => current.filter((row) => row.id !== item.id));
      setTotal((current) => Math.max(0, current - 1));
    } else {
      setRows((current) => current.map((row) => (row.id === item.id ? { ...row, read: true, readAt: row.readAt ?? new Date().toISOString() } : row)));
    }
    setUnreadCount((current) => Math.max(0, current - 1));
  };

  const openNotification = (item: HostNotification) => {
    if (!item.read) void markRead(item);
    if (isInternalLink(item.deepLink)) navigate(item.deepLink);
  };

  const markAllRead = async () => {
    if (markingAll) return;
    setMarkingAll(true);
    setError(null);
    setNotice(null);
    try {
      const result = await markAllNotificationsRead(client);
      if (result.error || !result.data) {
        setError(message(result.error));
        return;
      }
    } catch {
      setError(message(null));
      return;
    } finally {
      setMarkingAll(false);
    }
    setUnreadCount(0);
    setNotice(translate('markedAllRead'));
    if (filter === 'unread') {
      setRows([]);
      setTotal(0);
    } else {
      setRows((current) => current.map((row) => ({ ...row, read: true, readAt: row.readAt ?? new Date().toISOString() })));
    }
    setPage(1);
    setRefreshKey((current) => current + 1);
  };

  const changeFilter = (value: string) => {
    setFilter(value === 'unread' ? 'unread' : 'all');
    setPage(1);
    setNotice(null);
  };

  const changePage = (value: number) => {
    setPage(value);
    setNotice(null);
  };

  const rowModels: NotificationRowModel[] = rows.map((item) => ({
    id: item.id,
    title: item.title,
    body: item.body,
    timestamp: formatDateTime(item.createdAt, timezone),
    unread: !item.read,
    stateLabel: item.read ? translate('stateRead') : translate('stateUnread'),
    stateTone: item.read ? 'muted' : 'primary',
    typeLabel: label('types', item.type),
    priorityLabel: label('priorities', item.priority),
    priorityTone: priorityTone(item.priority),
    interactive: !item.read || isInternalLink(item.deepLink),
    navigable: isInternalLink(item.deepLink),
    onOpen: () => openNotification(item),
  }));

  return {
    rows: rowModels,
    rowCount: rowModels.length,
    filter,
    changeFilter,
    page,
    pageSize: PAGE_SIZE,
    total,
    changePage,
    unreadCount,
    loading,
    markingAll,
    error,
    notice,
    refreshKey,
    markAllRead,
    filterItems: [
      { value: 'all', label: translate('filterAll') },
      { value: 'unread', label: translate('filterUnread') },
    ],
    emptyTitle: filter === 'unread' ? translate('unreadEmpty') : translate('empty'),
    emptyHint: filter === 'unread' ? translate('unreadEmptyHint') : translate('emptyHint'),
    labels: {
      title: translate('title'),
      subtitle: translate('subtitle'),
      filter: translate('filterLabel'),
      unread: translate('unreadLabel'),
      markAllRead: translate('markAllRead'),
    },
  };
};

export type HostNotificationsView = ReturnType<typeof useHostNotifications>;
