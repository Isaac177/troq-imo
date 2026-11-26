import { useEffect, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { formatCdf, formatDateTime, zonedWallClock } from '@mungini/utils';
import { cancelTrip, getCancellationPreview, listCities, withdrawTrip } from '@/services/trips/trips.api';
import type { CancellationPreview, CancelledStay, TripDetail } from '@/types/trips/trips';

type ActionKind = 'cancel' | 'withdraw';

const failureKeys: Record<string, string> = {
  CANNOT_CANCEL: 'actionErrorCannotCancel',
  NO_CANCELLATION_DEADLINE: 'actionErrorNoDeadline',
  NO_CANCELLATION_POLICY: 'actionErrorNoPolicy',
  RESERVATION_VERSION_CONFLICT: 'actionErrorVersionConflict',
  REQUEST_IN_PROGRESS: 'actionErrorInProgress',
  CANNOT_WITHDRAW: 'actionErrorCannotWithdraw',
  APPROVAL_WINDOW_CLOSED: 'actionErrorWindowClosed',
};

const failureKey = (failure: unknown): string => {
  const code = failure instanceof Error ? failure.message : '';
  return failureKeys[code] ?? 'actionErrorGeneric';
};

const staleStates = ['CANNOT_CANCEL', 'CANNOT_WITHDRAW', 'APPROVAL_WINDOW_CLOSED', 'RESERVATION_VERSION_CONFLICT'];

const stateMoved = (failure: unknown): boolean => failure instanceof Error && staleStates.includes(failure.message);

export const useTripActions = (reservationId: string, trip: TripDetail | null, onChanged: () => void) => {
  const { client, city } = useSession();
  const { i18n } = useTranslation();
  const t = i18n.getFixedT(i18n.language, 'trips') as unknown as (key: string) => string;
  const [kind, setKind] = useState<ActionKind | null>(null);
  const [preview, setPreview] = useState<CancellationPreview | null>(null);
  const [previewLoading, setPreviewLoading] = useState(false);
  const [previewError, setPreviewError] = useState('');
  const [busy, setBusy] = useState(false);
  const [actionError, setActionError] = useState('');
  const [cancelled, setCancelled] = useState<CancelledStay | null>(null);
  const [withdrawn, setWithdrawn] = useState(false);
  const [idempotencyKey, setIdempotencyKey] = useState('');
  const [timezone, setTimezone] = useState('UTC');
  const today = zonedWallClock(new Date(), timezone)?.date ?? '';
  const started = Boolean(trip && today && trip.checkIn <= today);

  useEffect(() => {
    if (!city) return;
    let active = true;
    void listCities(client).then((items) => {
      const match = items.find((item) => item.code === city);
      if (active && match) setTimezone(match.timezone);
    }).catch(() => undefined);
    return () => { active = false; };
  }, [client, city]);

  const loadPreview = async () => {
    setPreviewLoading(true);
    setPreviewError('');
    try {
      setPreview(await getCancellationPreview(client, reservationId));
    } catch (failure) {
      setPreview(null);
      setPreviewError(t(failureKey(failure)));
    } finally {
      setPreviewLoading(false);
    }
  };

  const openCancel = () => {
    if (trip?.state !== 'CONFIRMED' || started) return;
    setKind('cancel');
    setActionError('');
    setPreview(null);
    setIdempotencyKey((current) => current || crypto.randomUUID());
    void loadPreview();
  };

  const openWithdraw = () => {
    if (trip?.state !== 'REQUESTED' && trip?.state !== 'PENDING_PAYMENT') return;
    setKind('withdraw');
    setActionError('');
  };

  const close = () => {
    if (busy) return;
    setKind(null);
    setPreview(null);
    setPreviewError('');
    setActionError('');
  };

  const confirmCancel = async () => {
    if (!trip || trip.state !== 'CONFIRMED' || started || !preview || !idempotencyKey || busy) return;
    setBusy(true);
    setActionError('');
    try {
      setCancelled(await cancelTrip(client, reservationId, trip.version, idempotencyKey));
      setKind(null);
      onChanged();
    } catch (failure) {
      setActionError(t(failureKey(failure)));
      if (failure instanceof Error && failure.message === 'RESERVATION_VERSION_CONFLICT') void loadPreview();
      if (stateMoved(failure)) {
        onChanged();
        if (!(failure instanceof Error && failure.message === 'RESERVATION_VERSION_CONFLICT')) setKind(null);
      }
    } finally {
      setBusy(false);
    }
  };

  const confirmWithdraw = async () => {
    if (!trip || trip.state !== 'REQUESTED' || busy) return;
    setBusy(true);
    setActionError('');
    try {
      await withdrawTrip(client, reservationId, trip.version);
      setWithdrawn(true);
      setKind(null);
      onChanged();
    } catch (failure) {
      setActionError(t(failureKey(failure)));
      if (stateMoved(failure)) {
        setKind(null);
        onChanged();
      }
    } finally {
      setBusy(false);
    }
  };

  const refunded = cancelled ? Number(cancelled.refund) > 0 : false;
  const money = (value: string) => formatCdf(value, i18n.language);
  const hasPenalty = preview ? Number(preview.penalty) > 0 : false;
  return {
    kind, busy, preview, previewLoading, previewError, actionError, cancelled, withdrawn,
    cancelOpen: kind === 'cancel', withdrawOpen: kind === 'withdraw',
    canCancel: trip?.state === 'CONFIRMED' && !started, canWithdraw: trip?.state === 'REQUESTED' || trip?.state === 'PENDING_PAYMENT', started, timezone,
    hasPenalty, refunded,
    cancelReady: Boolean(preview) && !previewLoading && trip?.state === 'CONFIRMED',
    withdrawDescription: actionError || t('withdrawIntro'),
    inlineError: kind === null ? actionError : '',
    completed: Boolean(cancelled) || withdrawn,
    openCancel, openWithdraw, close, retryPreview: loadPreview, confirmCancel, confirmWithdraw,
    onOpenChange: (open: boolean) => { if (!open) close(); },
    money,
    penaltyText: preview ? (hasPenalty ? money(preview.penalty) : t('cancelNoCharge')) : '',
    deadline: preview ? formatDateTime(preview.deadline, timezone) : '',
    chargeDue: cancelled ? money(cancelled.chargeDueAtProperty) : '',
    refund: cancelled ? money(cancelled.refund) : '',
    labels: {
      cancel: t('cancelTrip'), withdraw: t('withdrawRequest'),
      cancelTitle: t('cancelTitle'), cancelIntro: t('cancelIntro'), termsLoading: t('termsLoading'),
      confirmCancel: t('confirmCancel'), keepTrip: t('keepTrip'),
      withdrawTitle: t('withdrawTitle'),
      confirmWithdraw: t('confirmWithdraw'), keepRequest: t('keepRequest'),
      total: t('cancelTotal'), refundLabel: t('cancelRefund'), penalty: t('cancelPenalty'),
      deadlineLabel: t('cancelDeadline'),
      cancelSuccess: t('cancelSuccess'), chargeDue: t('cancelSuccessCharge'), refundNote: t('cancelSuccessRefund'),
      withdrawSuccess: t('withdrawSuccess'), retry: t('retry'),
    },
  };
};

export type TripActionsView = ReturnType<typeof useTripActions>;
