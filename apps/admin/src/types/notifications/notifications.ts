import type { components } from '@mungini/api';

export type AdminNotification = components['schemas']['NotificationView'];
export type NotificationPage = components['schemas']['Envelope_Page_NotificationView']['data'];
export type UnreadCount = components['schemas']['UnreadCount'];
export type MarkedCount = components['schemas']['MarkedCount'];
export type RequestResult<T> = { data: T | null; error: string | null };
export type NotificationFilter = 'all' | 'unread';
