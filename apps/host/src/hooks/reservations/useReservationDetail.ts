import { useEffect, useState } from 'react';
import type { CalendarRange } from '@mungini/ui-web';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { formatCdf, formatIsoDay } from '@mungini/utils';
import {
  cancelReservation,
  checkInReservation,
  checkOutReservation,
  decideReservation,
  decideSpecialRequest,
  getReservation,
  noShowReservation,
  proposeAmendment,
} from '@/services/reservations/reservations.api';
import type { HostReservation, RequestResult } from '@/types/reservations/reservations';

const KNOWN_ERRORS = [
  'RESERVATION_VERSION_CONFLICT',
  'RESERVATION_STATE_INVALID',
  'CANNOT_CANCEL',
  'REQUEST_IN_PROGRESS',
  'FORBIDDEN',
  'NO_NIGHTS_AVAILABLE',
  'RESERVATION_NOT_FOUND',
  'CANCEL_REASON_REQUIRED',
  'CANCEL_REASON_TOO_LONG',
  'SPECIAL_REQUEST_NOT_PENDING',
];

const emptyRange: CalendarRange = { start: null, end: null };

export const useReservationDetail = (reservationId: string | null, onChanged: () => void) => {
  const { t, i18n } = useTranslation('hostReservations');
  const translate = t as unknown as (key: string) => string;
  const { client, can: canDo } = useSession();
  const locale = i18n.language.startsWith('fr') ? 'fr' : 'en';
  const [detail, setDetail] = useState<HostReservation | null>(null);
  const [loading, setLoading] = useState(false);
  const [loadError, setLoadError] = useState(false);
  const [busy, setBusy] = useState<string | null>(null);
  const [actionError, setActionError] = useState<string | null>(null);
  const [notice, setNotice] = useState<string | null>(null);
  const [confirmAction, setConfirmAction] = useState<'DECLINE' | 'NO_SHOW' | null>(null);
  const [cancelOpen, setCancelOpen] = useState(false);
  const [reason, setReason] = useState('');
  const [cancelBusy, setCancelBusy] = useState(false);
  const [cancelError, setCancelError] = useState<string | null>(null);
  const [amendOpen, setAmendOpen] = useState(false);
  const [range, setRange] = useState<CalendarRange>(emptyRange);
  const [amendBusy, setAmendBusy] = useState(false);
  const [amendError, setAmendError] = useState<string | null>(null);
  const [reloadToken, setReloadToken] = useState(0);

  const message = (code: string | null) => (code && KNOWN_ERRORS.includes(code) ? translate(`errors.${code}`) : translate('errors.generic'));

  useEffect(() => {
    setDetail(null);
    setActionError(null);
    setNotice(null);
    setConfirmAction(null);
    setCancelOpen(false);
    setAmendOpen(false);
    setReason('');
    setRange(emptyRange);
  }, [reservationId]);

  useEffect(() => {
    if (!reservationId) return;
    let active = true;
    setLoading(true);
    setLoadError(false);
    void getReservation(client, reservationId, locale).then((result) => {
      if (!active) return;
      setLoading(false);
      if (result.error || !result.data) {
        setLoadError(true);
        return;
      }
      setDetail(result.data);
    });
    return () => {
      active = false;
    };
  }, [client, reservationId, locale, reloadToken]);

  const run = async (key: string, action: () => Promise<RequestResult<unknown>>) => {
    setBusy(key);
    setActionError(null);
    setNotice(null);
    const result = await action();
    setBusy(null);
    if (result.error) {
      setActionError(message(result.error));
      return false;
    }
    setNotice(translate('updated'));
    setReloadToken((value) => value + 1);
    onChanged();
    return true;
  };

  const accept = () => detail && void run('ACCEPT', () => decideReservation(client, detail.id, 'ACCEPT', detail.version));
  const decline = async () => {
    if (!detail) return;
    if (await run('DECLINE', () => decideReservation(client, detail.id, 'DECLINE', detail.version))) setConfirmAction(null);
  };
  const checkIn = () => detail && void run('CHECK_IN', () => checkInReservation(client, detail.id, detail.version));
  const checkOut = () => detail && void run('CHECK_OUT', () => checkOutReservation(client, detail.id, detail.version));
  const noShow = async () => {
    if (!detail) return;
    if (await run('NO_SHOW', () => noShowReservation(client, detail.id, detail.version))) setConfirmAction(null);
  };
  const acceptRequest = () => detail && void run('ACCEPT_REQUEST', () => decideSpecialRequest(client, detail.id, 'ACCEPT', detail.version));
  const declineRequest = () => detail && void run('DECLINE_REQUEST', () => decideSpecialRequest(client, detail.id, 'DECLINE', detail.version));

  const submitCancel = async () => {
    if (!detail) return;
    setCancelBusy(true);
    setCancelError(null);
    const result = await cancelReservation(client, detail.id, detail.version, reason);
    setCancelBusy(false);
    if (result.error) {
      setCancelError(message(result.error));
      return;
    }
    setCancelOpen(false);
    setReason('');
    setNotice(translate('updated'));
    setReloadToken((value) => value + 1);
    onChanged();
  };

  const submitAmend = async () => {
    if (!detail || !range.start || !range.end) return;
    setAmendBusy(true);
    setAmendError(null);
    const result = await proposeAmendment(client, detail.id, range.start, range.end);
    setAmendBusy(false);
    if (result.error) {
      setAmendError(message(result.error));
      return;
    }
    setAmendOpen(false);
    setRange(emptyRange);
    setNotice(translate('amendDone'));
    setReloadToken((value) => value + 1);
    onChanged();
  };

  const can = (action: string) => detail?.allowedActions.includes(action) ?? false;
  const nights = (detail?.nights ?? []).map((night) => ({ date: formatIsoDay(night.date), amount: formatCdf(night.amount, i18n.language) }));

  return {
    detail,
    loading,
    loadError,
    busy,
    actionError,
    notice,
    confirmAction,
    setConfirmAction,
    accept,
    decline,
    checkIn,
    checkOut,
    noShow,
    acceptRequest,
    declineRequest,
    can,
    nights,
    cancelOpen,
    setCancelOpen,
    reason,
    setReason,
    cancelBusy,
    cancelError,
    submitCancel,
    amendOpen,
    setAmendOpen,
    range,
    setRange,
    amendBusy,
    amendError,
    submitAmend,
    money: (amount: string) => formatCdf(amount, i18n.language),
    canMessage: canDo('messages.write'),
    day: (value: string | null | undefined) => formatIsoDay(value),
    rate: detail ? `${(detail.commissionRateBps / 100).toFixed(2)}%` : '',
    stateLabel: (value: string) => translate(`states.${value}`),
    requestStateLabel: (value: string) => translate(`specialRequestStates.${value}`),
    labels: {
      title: translate('detailTitle'),
      status: translate('colStatus'),
      reference: translate('reference'),
      property: translate('property'),
      unit: translate('unit'),
      assignedUnit: translate('assignedUnit'),
      stay: translate('stay'),
      nights: translate('nights'),
      guest: translate('guest'),
      contactEmail: translate('contactEmail'),
      contactPhone: translate('contactPhone'),
      adults: translate('adults'),
      children: translate('children'),
      specialRequest: translate('specialRequest'),
      specialRequestState: translate('specialRequestState'),
      approvalDeadline: translate('approvalDeadline'),
      createdAt: translate('createdAt'),
      total: translate('total'),
      commission: translate('commission'),
      commissionRate: translate('commissionRate'),
      net: translate('net'),
      actions: translate('actions'),
      accept: translate('accept'),
      decline: translate('decline'),
      checkIn: translate('checkIn'),
      checkOut: translate('checkOut'),
      noShow: translate('noShow'),
      cancelReservation: translate('cancelReservation'),
      acceptSpecialRequest: translate('acceptSpecialRequest'),
      declineSpecialRequest: translate('declineSpecialRequest'),
      modifyDates: translate('modifyDates'),
      guestHidden: translate('guestHidden'),
      error: translate('error'),
      updated: translate('updated'),
      confirmDeclineTitle: translate('confirmDeclineTitle'),
      confirmDeclineBody: translate('confirmDeclineBody'),
      confirmNoShowTitle: translate('confirmNoShowTitle'),
      confirmNoShowBody: translate('confirmNoShowBody'),
      cancelTitle: translate('cancelTitle'),
      cancelBody: translate('cancelBody'),
      cancelReason: translate('cancelReason'),
      cancelReasonPlaceholder: translate('cancelReasonPlaceholder'),
      cancelConfirm: translate('cancelConfirm'),
      dismiss: translate('dismiss'),
      amendTitle: translate('amendTitle'),
      amendBody: translate('amendBody'),
      amendDates: translate('amendDates'),
      amendSubmit: translate('amendSubmit'),
      close: translate('close'),
    },
  };
};
