import type { AdminReservations } from '../en/adminReservations';
export const adminReservations = {
  all: 'Tous les statuts', status: 'Statut', details: 'Détails de la réservation', loading: 'Chargement de la réservation', error: 'Impossible de charger la réservation',
  host: 'Hôte', property: 'Hébergement', guest: 'Voyageur', amount: 'Total', commission: 'Commission', net: 'Montant de l’hôte',
  stays: 'Séjours', guests: 'Voyageurs', cases: 'Dossiers', amendments: 'Modifications', timeline: 'Historique', empty: 'Aucun',
  previous: 'Précédent', next: 'Suivant', page: 'Page', children: 'Enfants', adults: 'Adultes', quantity: 'Quantité',
  pendingPayment: 'En attente de paiement', requested: 'Demandée', confirmed: 'Confirmée', declined: 'Refusée', cancelled: 'Annulée',
  expired: 'Expirée', checkedIn: 'Arrivée effectuée', checkedOut: 'Départ effectué', noShow: 'Non présenté', other: 'Autre',
} satisfies AdminReservations;
