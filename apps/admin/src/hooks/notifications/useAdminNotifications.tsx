import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import type { StatusTone } from '@mungini/theme/tones';
import { StatusDot, TonePill, type TableColumn } from '@mungini/ui-web';
import { formatDateTime } from '@mungini/utils';
import NotificationSummaryCell from '@/components/notifications/NotificationSummaryCell';
import { getUnreadCount, listCities, listNotifications, markAllNotificationsRead, markNotificationRead } from '@/services/notifications/notifications.api';
import type { AdminNotification, NotificationFilter } from '@/types/notifications/notifications';

const PAGE_SIZE = 20;
const KNOWN_ERRORS = ['NOTIFICATION_NOT_FOUND', 'LOCALE_INVALID', 'FORBIDDEN'];

const isInternalLink = (link: string | null | undefined): link is string => Boolean(link && link.startsWith('/') && !link.startsWith('//'));

const priorityTone = (priority: string): StatusTone => {
  if (priority === 'URGENT') return 'danger';
  if (priority === 'HIGH') return 'warning';
  if (priority === 'NORMAL') return 'neutral';
  return 'muted';
};

export const useAdminNotifications = () => {
  const { t, i18n } = useTranslation('adminNotifications');
  const translate = t as unknown as (key: string, options?: Record<string, unknown>) => string;
  const { client, city } = useSession();
  const navigate = useNavigate();
  const [filter, setFilter] = useState<NotificationFilter>('all');
  const [page, setPage] = useState(1);
  const [rows, setRows] = useState<AdminNotification[]>([]);
  const [total, setTotal] = useState(0);
  const [unreadCount, setUnreadCount] = useState(0);
  const [loading, setLoading] = useState(true);
  const [markingAll, setMarkingAll] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [actionError, setActionError] = useState<string | null>(null);
  const [notice, setNotice] = useState<string | null>(null);
  const [refreshKey, setRefreshKey] = useState(0);
  const [timezone, setTimezone] = useState('UTC');
  const locale = i18n.language.startsWith('fr') ? 'fr' : 'en';

  const message = (code: string | null) => (code && KNOWN_ERRORS.includes(code) ? translate(`errors.${code}`) : translate('errors.generic'));
  const label = (group: 'types' | 'priorities', value: string) => translate(`${group}.${value}`, { defaultValue: value });
  const stateLabel = (read: boolean) => (read ? translate('stateRead') : translate('stateUnread'));

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
  }, [client, city, refreshKey]);

  useEffect(() => {
    setPage(1);
  }, [city]);

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
  }, [client, city, locale, filter, page, refreshKey, translate]);

  useEffect(() => {
    if (!loading && !error && rows.length === 0 && page > 1) setPage(total === 0 ? 1 : page - 1);
  }, [loading, error, rows.length, page, total]);

  const markRead = async (item: AdminNotification) => {
    setActionError(null);
    try {
      const result = await markNotificationRead(client, item.id);
      if (result.error || !result.data) {
        setActionError(message(result.error));
        return;
      }
    } catch {
      setActionError(message(null));
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

  const openNotification = (item: AdminNotification) => {
    if (!item.read) void markRead(item);
    if (isInternalLink(item.deepLink)) navigate(item.deepLink);
  };

  const markAllRead = async () => {
    if (markingAll) return;
    setMarkingAll(true);
    setActionError(null);
    setNotice(null);
    try {
      const result = await markAllNotificationsRead(client);
      if (result.error || !result.data) {
        setActionError(message(result.error));
        return;
      }
    } catch {
      setActionError(message(null));
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
    setActionError(null);
  };

  const changePage = (value: number) => {
    setPage(value);
    setNotice(null);
  };

  const columns: TableColumn<AdminNotification>[] = [
    {
      key: 'notification',
      header: translate('columnNotification'),
      cell: (row) => (
        <NotificationSummaryCell
          title={row.title}
          body={row.body}
          typeLabel={label('types', row.type)}
          priorityLabel={label('priorities', row.priority)}
          timestamp={formatDateTime(row.createdAt, timezone)}
          unread={!row.read}
        />
      ),
    },
    { key: 'type', header: translate('columnType'), hideBelow: 'sm', cell: (row) => label('types', row.type) },
    {
      key: 'priority',
      header: translate('columnPriority'),
      hideBelow: 'sm',
      cell: (row) => <TonePill tone={priorityTone(row.priority)} label={label('priorities', row.priority)} />,
    },
    {
      key: 'state',
      header: translate('columnState'),
      cell: (row) => <StatusDot tone={row.read ? 'muted' : 'primary'} label={stateLabel(row.read)} />,
    },
    {
      key: 'date',
      header: translate('columnDate'),
      hideBelow: 'sm',
      cell: (row) => <span className="whitespace-nowrap tabular-nums">{formatDateTime(row.createdAt, timezone)}</span>,
    },
  ];

  return {
    rows,
    columns,
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
    actionError,
    notice,
    markAllRead,
    open: openNotification,
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

export type AdminNotificationsView = ReturnType<typeof useAdminNotifications>;
