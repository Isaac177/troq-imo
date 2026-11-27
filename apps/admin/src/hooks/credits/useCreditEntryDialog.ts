import { useEffect, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { adjustCredit, topUpCredit } from '@/services/credits/credits.api';
import type { CreditDirection, CreditEntryMode } from '@/types/credits/credits';
import { useCreditErrorMessage } from '@/hooks/credits/useCreditErrorMessage';

const TOPUP_REASONS = ['CASH_RECEIVED', 'MOBILE_MONEY_RECEIVED', 'BANK_TRANSFER', 'PROMOTIONAL_CREDIT'];
const ADJUST_REASONS = ['ERROR_CORRECTED', 'GOODWILL', 'CHARGEBACK_REVERSAL'];
const DIRECTIONS: CreditDirection[] = ['CREDIT', 'DEBIT'];
const MAX_AMOUNT = 100_000_000;

const isValidAmount = (value: string) => /^\d{1,9}(\.\d{1,2})?$/.test(value.trim()) && Number(value) > 0 && Number(value) <= MAX_AMOUNT;

export const useCreditEntryDialog = (hostId: string | null, mode: CreditEntryMode, open: boolean, onClose: () => void, onSaved: () => void) => {
  const { t } = useTranslation('adminCredits');
  const translate = t as unknown as (key: string) => string;
  const errorMessage = useCreditErrorMessage();
  const { client } = useSession();
  const [amount, setAmount] = useState('');
  const [reasonCode, setReasonCode] = useState('');
  const [direction, setDirection] = useState<CreditDirection>('CREDIT');
  const [paymentRef, setPaymentRef] = useState('');
  const [note, setNote] = useState('');
  const [busy, setBusy] = useState(false);
  const [formError, setFormError] = useState<string | null>(null);

  useEffect(() => {
    setAmount('');
    setReasonCode('');
    setDirection('CREDIT');
    setPaymentRef('');
    setNote('');
    setBusy(false);
    setFormError(null);
  }, [open, mode, hostId]);

  const tr = (key: string) => translate(`${mode}.${key}`);
  const reasons = mode === 'topup' ? TOPUP_REASONS : ADJUST_REASONS;
  const showReference = mode === 'topup';
  const needsReference = mode === 'topup' && reasonCode !== '' && reasonCode !== 'PROMOTIONAL_CREDIT';
  const canSubmit = Boolean(hostId) && isValidAmount(amount) && reasonCode !== '' && (!needsReference || paymentRef.trim() !== '') && !busy;

  const submit = async () => {
    if (!hostId || !canSubmit) return;
    setBusy(true);
    setFormError(null);
    const result = mode === 'topup'
      ? await topUpCredit(client, hostId, { amount: amount.trim(), reasonCode, paymentRef: paymentRef.trim() || null, note: note.trim() || null })
      : await adjustCredit(client, hostId, { amount: amount.trim(), direction, reasonCode, note: note.trim() || null });
    setBusy(false);
    if (result.error || !result.data) {
      setFormError(errorMessage(result.error));
      return;
    }
    onClose();
    onSaved();
  };

  return {
    amount,
    setAmount: (value: string) => {
      setFormError(null);
      setAmount(value);
    },
    reasonCode,
    setReasonCode: (value: string) => {
      setFormError(null);
      setReasonCode(value);
    },
    direction,
    setDirection: (value: string) => setDirection(value as CreditDirection),
    paymentRef,
    setPaymentRef,
    note,
    setNote,
    showReference,
    busy,
    formError,
    canSubmit,
    submit,
    reasonOptions: reasons.map((value) => ({ value, label: translate(`reasons.${value}`) })),
    directionOptions: DIRECTIONS.map((value) => ({ value, label: translate(`directions.${value}`) })),
    labels: {
      title: tr('title'), description: tr('description'), direction: mode === 'adjust' ? tr('direction') : '',
      amount: tr('amount'), amountHint: tr('amountHint'),
      reason: tr('reason'), chooseReason: tr('chooseReason'),
      paymentRef: showReference ? tr('paymentRef') : '', paymentRefHint: showReference ? tr('paymentRefHint') : '',
      note: tr('note'), noteHint: tr('noteHint'), submit: tr('submit'), cancel: tr('cancel'),
    },
  };
};

export type CreditEntryDialogView = ReturnType<typeof useCreditEntryDialog>;
