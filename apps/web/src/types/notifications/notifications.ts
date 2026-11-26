import type { components } from '@mungini/api';

export type Notification = components['schemas']['NotificationView'];
export type NotificationPage = components['schemas']['Envelope_Page_NotificationView']['data'];
export type NotificationCity = components['schemas']['CityView'];
export type NotificationFilter = 'all' | 'unread';

export type NotificationRowView = {
  id: string;
  title: string;
  body: string;
  typeLabel: string;
  priorityLabel: string;
  readLabel: string;
  timestamp: string;
  read: boolean;
  tone: 'primary' | 'muted';
  navigable: boolean;
  interactive: boolean;
  open: () => void;
};
