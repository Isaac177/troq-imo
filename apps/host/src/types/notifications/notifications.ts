import type { components } from '@mungini/api';
import type { StatusTone } from '@mungini/theme/tones';

export type HostNotification = components['schemas']['NotificationView'];
export type NotificationPage = components['schemas']['Envelope_Page_NotificationView']['data'];
export type UnreadCount = components['schemas']['UnreadCount'];
export type MarkedCount = components['schemas']['MarkedCount'];
export type RequestResult<T> = { data: T | null; error: string | null };
export type NotificationFilter = 'all' | 'unread';

export type NotificationRowModel = {
  id: string;
  title: string;
  body: string;
  timestamp: string;
  unread: boolean;
  stateLabel: string;
  stateTone: StatusTone;
  typeLabel: string;
  priorityLabel: string;
  priorityTone: StatusTone;
  interactive: boolean;
  navigable: boolean;
  onOpen: () => void;
};
