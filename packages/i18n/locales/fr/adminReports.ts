import type { AdminReports } from '../en/adminReports';

export const adminReports = {
  title: 'Rapports',
  subtitle: 'Résultats des réservations et avis pour la ville sélectionnée, selon la date d’arrivée.',
  dates: 'Période d’arrivée',
  error: 'Impossible de charger le rapport.',
  revenue: 'Valeur brute des réservations',
  reservations: 'Réservations',
  confirmed: 'Confirmées, y compris les séjours en cours ou terminés',
  checkedIn: 'Arrivées',
  checkedOut: 'Départs',
  cancelled: 'Annulées',
  noShow: 'Non-présentations',
  reviews: 'Avis publiés',
  rating: 'Note moyenne',
  unavailable: 'Aucune note',
} satisfies AdminReports;
