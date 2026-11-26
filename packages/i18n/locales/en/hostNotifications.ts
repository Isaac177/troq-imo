export const hostNotifications = {
  title: 'Notifications',
  subtitle: 'Reservation, property and case updates for the stays you manage.',
  filterLabel: 'Notification filter',
  filterAll: 'All',
  filterUnread: 'Unread',
  unreadLabel: 'Unread',
  stateUnread: 'Unread',
  stateRead: 'Read',
  markAllRead: 'Mark all read',
  markedAllRead: 'All notifications marked as read.',
  empty: 'No notifications yet.',
  emptyHint: 'Updates about your stays will appear here.',
  unreadEmpty: 'No unread notifications.',
  unreadEmptyHint: 'You have read every notification.',
  error: 'Notifications could not be loaded.',
  types: {
    INFO: 'Information',
    ALERT: 'Alert',
    REMINDER: 'Reminder',
    MESSAGE: 'Message',
    REVIEW_RECEIVED: 'Review',
  },
  priorities: {
    NORMAL: 'Normal',
    HIGH: 'High',
    URGENT: 'Urgent',
  },
  errors: {
    generic: 'The action could not be completed.',
    NOTIFICATION_NOT_FOUND: 'This notification no longer exists.',
    FORBIDDEN: 'You do not have permission for this action.',
  },
};

export type HostNotificationsLocale = typeof hostNotifications;
