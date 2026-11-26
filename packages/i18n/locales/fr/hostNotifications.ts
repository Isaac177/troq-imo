import type { HostNotificationsLocale } from '../en/hostNotifications';

export const hostNotifications = {
  title: 'Notifications',
  subtitle: 'Mises à jour des réservations, hébergements et dossiers pour les séjours que vous gérez.',
  filterLabel: 'Filtre des notifications',
  filterAll: 'Toutes',
  filterUnread: 'Non lues',
  unreadLabel: 'Non lues',
  stateUnread: 'Non lue',
  stateRead: 'Lue',
  markAllRead: 'Tout marquer comme lu',
  markedAllRead: 'Toutes les notifications sont marquées comme lues.',
  empty: 'Aucune notification pour le moment.',
  emptyHint: 'Les mises à jour de vos séjours apparaîtront ici.',
  unreadEmpty: 'Aucune notification non lue.',
  unreadEmptyHint: 'Vous avez lu toutes vos notifications.',
  error: 'Les notifications n’ont pas pu être chargées.',
  types: {
    INFO: 'Information',
    ALERT: 'Alerte',
    REMINDER: 'Rappel',
    MESSAGE: 'Message',
    REVIEW_RECEIVED: 'Avis',
  },
  priorities: {
    NORMAL: 'Normale',
    HIGH: 'Élevée',
    URGENT: 'Urgente',
  },
  errors: {
    generic: 'L’action n’a pas pu être effectuée.',
    NOTIFICATION_NOT_FOUND: 'Cette notification n’existe plus.',
    FORBIDDEN: 'Vous n’avez pas la permission pour cette action.',
  },
} satisfies HostNotificationsLocale;
