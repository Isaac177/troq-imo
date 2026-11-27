import { useCallback, useEffect, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { listReservationChoices, openCase } from '@/services/cases/cases.api';
import { HOST_CASE_TYPES, type CaseOption, type HostCaseType } from '@/types/cases/cases';

const KNOWN_ERRORS = [
  'RESERVATION_NOT_FOUND',
  'HOST_NOT_FOUND',
  'HOST_REQUIRED',
  'CASE_HOST_MISMATCH',
  'CASE_HOST_REQUIRED',
  'CASE_RESERVATION_REQUIRED',
  'CASE_TYPE_NOT_ALLOWED',
  'CASE_TYPE_NEEDS_PAYMENT_RULES',
  'CASE_REASON_REQUIRED',
  'CASE_REASON_TOO_LONG',
  'CLAIM_AMOUNT_REQUIRED',
  'CLAIM_AMOUNT_INVALID',
  'CLAIM_AMOUNT_TOO_HIGH',
  'CLAIM_BEFORE_CHECKOUT',
  'CLAIM_WINDOW_CLOSED',
  'DAMAGE_CLAIM_NEEDS_CHECKOUT',
  'FORBIDDEN',
];

const NAMESPACE = 'hostCases';

type Options = {
  hostId: string;
  canWrite: boolean;
  onOpened: () => void;
};

export const useOpenCase = ({ hostId, canWrite, onOpened }: Options) => {
  const { t, i18n } = useTranslation();
  const translateRaw = t as unknown as (key: string, options?: Record<string, unknown>) => string;
  const translate = useCallback((key: string, options?: Record<string, unknown>) => translateRaw(key, { ...options, ns: NAMESPACE }), [translateRaw]);
  const { client } = useSession();
  const [open, setOpen] = useState(false);
  const [caseType, setCaseType] = useState<HostCaseType>(HOST_CASE_TYPES[0] ?? 'OPERATOR_COMPLAINT');
  const [reservationId, setReservationId] = useState('');
  const [reason, setReason] = useState('');
  const [claimAmount, setClaimAmount] = useState('');
  const [busy, setBusy] = useState(false);
  const [formError, setFormError] = useState('');
  const [reservations, setReservations] = useState<CaseOption[]>([]);
  const [loadingReservations, setLoadingReservations] = useState(false);
  const [reservationsError, setReservationsError] = useState('');
  const locale = i18n.language.startsWith('fr') ? 'fr' : 'en';

  const message = (code: string | null) => (code && KNOWN_ERRORS.includes(code) ? translate(`errors.${code}`) : translate('errors.generic'));

  useEffect(() => {
    if (!open || !hostId) return;
    let active = true;
    setLoadingReservations(true);
    setReservationsError('');
    void listReservationChoices(client, hostId, locale).then((result) => {
      if (!active) return;
      setLoadingReservations(false);
      if (result.error || !result.data) {
        setReservations([]);
        setReservationsError(message(result.error));
        return;
      }
      setReservations(result.data);
    }).catch(() => {
      if (!active) return;
      setLoadingReservations(false);
      setReservations([]);
      setReservationsError(translate('error'));
    });
    return () => {
      active = false;
    };
  }, [client, hostId, open, locale, translate]);

  const openDialog = () => {
    if (!canWrite || !hostId) return;
    setCaseType(HOST_CASE_TYPES[0] ?? 'OPERATOR_COMPLAINT');
    setReservationId('');
    setReason('');
    setClaimAmount('');
    setFormError('');
    setReservationsError('');
    setOpen(true);
  };

  const onOpenChange = (next: boolean) => {
    if (!next && busy) return;
    setOpen(next);
  };

  const close = () => {
    if (busy) return;
    setOpen(false);
  };

  const submit = async () => {
    if (!hostId || !reservationId || reason.trim().length === 0) return;
    if (caseType === 'DAMAGE_CLAIM' && !(Number(claimAmount) > 0)) {
      setFormError(translate('errors.CLAIM_AMOUNT_INVALID'));
      return;
    }
    setBusy(true);
    setFormError('');
    const result = await openCase(client, {
      caseType,
      hostId,
      reservationId,
      reason: reason.trim(),
      ...(caseType === 'DAMAGE_CLAIM' ? { claimAmount } : {}),
    });
    setBusy(false);
    if (result.error || !result.data) {
      setFormError(message(result.error));
      return;
    }
    setOpen(false);
    onOpened();
  };

  const canSubmit = Boolean(hostId) && Boolean(reservationId) && reason.trim().length > 0
    && (caseType !== 'DAMAGE_CLAIM' || Number(claimAmount) > 0)
    && !busy;

  return {
    open,
    openDialog,
    onOpenChange,
    close,
    caseType,
    selectCaseType: (value: string) => setCaseType(value as HostCaseType),
    reservationId,
    setReservationId,
    reason,
    setReason,
    claimAmount,
    setClaimAmount,
    busy,
    formError,
    canSubmit,
    submit,
    reservationOptions: reservations,
    loadingReservations,
    reservationsError,
    typeOptions: HOST_CASE_TYPES.map((value) => ({ value, label: translate(`types.${value}`) })),
    labels: {
      title: translate('openTitle'),
      description: translate('openHelp'),
      reservation: translate('reservation'),
      selectReservation: translate('selectReservation'),
      searchReservation: translate('searchReservation'),
      noReservations: translate('noReservations'),
      caseType: translate('caseType'),
      reason: translate('reasonLabel'),
      reasonPlaceholder: translate('reasonPlaceholder'),
      claimAmount: translate('claimAmount'),
      claimAmountHint: translate('claimAmountHint'),
      amountPlaceholder: translate('amountPlaceholder'),
      submit: translate('openCase'),
      cancel: translate('dismiss'),
    },
  };
};

export type OpenCaseView = ReturnType<typeof useOpenCase>;
