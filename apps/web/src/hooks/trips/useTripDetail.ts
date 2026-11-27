import { useEffect, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { formatCdf, formatDateTime, formatIsoDay } from '@mungini/utils';
import { getTrip } from '@/services/trips/trips.api';
import { useTripActions } from '@/hooks/trips/useTripActions';
import type { TripDetail } from '@/types/trips/trips';

const keys: Record<string, string> = { REQUESTED: 'requested', PENDING_PAYMENT: 'pendingPayment', CONFIRMED: 'confirmed', CHECKED_IN: 'checkedIn', CHECKED_OUT: 'checkedOut', NO_SHOW: 'noShow', COMPLETED: 'completed', CANCELLED: 'cancelledState', EXPIRED: 'expired', DECLINED: 'declined', HELD: 'held' };

export const useTripDetail = (reservationId: string) => {
  const { client, status, city } = useSession();
  const { i18n } = useTranslation();
  const t = i18n.getFixedT(i18n.language, 'trips') as unknown as (key: string) => string;
  const [trip, setTrip] = useState<TripDetail | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(false);
  const [refresh, setRefresh] = useState(0);
  const [photoIndex, setPhotoIndex] = useState(0);
  const photos = (trip?.unitTypeImages ?? []).map((photo) => ({ id: photo.id, url: photo.url, alt: (i18n.language.startsWith('fr') ? photo.altFr || photo.altEn : photo.altEn || photo.altFr) || trip?.reference || '' }));
  if (!photos.length && trip?.unitTypeCoverUrl) photos.push({ id: 'cover', url: trip.unitTypeCoverUrl, alt: trip.reference });
  const photo = photos[photoIndex] ?? photos[0];
  const previousPhoto = () => setPhotoIndex((index) => (index + photos.length - 1) % photos.length);
  const nextPhoto = () => setPhotoIndex((index) => (index + 1) % photos.length);
  const retry = () => setRefresh((value) => value + 1);
  const actions = useTripActions(reservationId, trip, retry);
  useEffect(() => {
    setPhotoIndex(0);
    if (status !== 'authenticated') return;
    let active = true;
    setLoading(true);
    setTrip(null);
    void getTrip(client, reservationId).then((data) => { if (active) { setTrip(data); setError(false); } })
      .catch(() => { if (active) setError(true); }).finally(() => { if (active) setLoading(false); });
    return () => { active = false; };
  }, [client, city, status, reservationId, refresh]);
  return {
    status, trip, loading, error, actions, retry,
    state: trip ? t(keys[trip.state] ?? 'unknown') : '',
    checkIn: trip ? formatIsoDay(trip.checkIn) : '', checkOut: trip ? formatIsoDay(trip.checkOut) : '',
    total: trip ? formatCdf(trip.total, i18n.language) : '',
    discount: trip?.discount ? `- ${formatCdf(trip.discount.amount, i18n.language)}` : '',
    nights: trip ? trip.nights.length : 0,
    nightLines: (trip?.nights ?? []).map((night) => ({ date: formatIsoDay(night.date), amount: formatCdf(night.amount, i18n.language) })),
    approvalDeadline: trip?.approvalDeadlineAt ? formatDateTime(trip.approvalDeadlineAt, actions.timezone) : '',
    holdExpiry: trip?.holdExpiresAt ? formatDateTime(trip.holdExpiresAt, actions.timezone) : '',
    stateInfo: trip ? (() => {
      const key = `stateInfo.${trip.state}`;
      const value = t(key);
      return value === key ? '' : value;
    })() : '',
    contactEmail: trip?.guest.contactEmail ?? '',
    contactPhone: trip?.guest.contactPhone ?? '',
    photo, hasMultiplePhotos: photos.length > 1, previousPhoto, nextPhoto,
    propertyHref: trip ? `/stays/${trip.propertyId}` : '',
    labels: {
      title: t('detailTitle'), back: t('backToTrips'), signIn: t('signIn'), signInPrompt: t('signInPrompt'), loading: t('loading'), error: t('error'), retry: t('retry'), reference: t('reference'), status: t('status'), checkIn: t('checkIn'), checkOut: t('checkOut'), total: t('total'), guest: t('guest'), adults: t('adults'), children: t('children'), request: t('specialRequest'),
      nights: t('nights'), nightlyRate: t('nightlyRate'), discount: t('discount'), approvalDeadline: t('approvalDeadline'), holdExpiry: t('holdExpiry'), contact: t('contact'), viewListing: t('viewListing'), started: t('started'), previousPhoto: t('previousPhoto'), nextPhoto: t('nextPhoto'),
    },
  };
};
