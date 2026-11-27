import { useEffect, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { setCommissionRate } from '@/services/credits/credits.api';
import { useCreditErrorMessage } from '@/hooks/credits/useCreditErrorMessage';

const RATE_REASONS = ['CONTRACT_AGREED', 'RENEGOTIATED', 'LAUNCH_OFFER', 'PLATFORM_POLICY', 'ERROR_CORRECTED'];

const isValidRate = (value: string) => /^\d{1,3}(\.\d{1,2})?$/.test(value.trim()) && Number(value) >= 0 && Number(value) <= 100;

export const useCommissionRateDialog = (hostId: string | null, currentBps: number, open: boolean, onClose: () => void, onSaved: () => void) => {
  const { t, i18n } = useTranslation('adminCredits');
  const translate = t as unknown as (key: string) => string;
  const errorMessage = useCreditErrorMessage();
  const { client } = useSession();
  const [rate, setRate] = useState('');
  const [reasonCode, setReasonCode] = useState('');
  const [note, setNote] = useState('');
  const [busy, setBusy] = useState(false);
  const [formError, setFormError] = useState<string | null>(null);

  useEffect(() => {
    setRate('');
    setReasonCode('');
    setNote('');
    setBusy(false);
    setFormError(null);
  }, [open, hostId]);

  const percent = (bps: number) => new Intl.NumberFormat(i18n.language, { style: 'percent', minimumFractionDigits: 0, maximumFractionDigits: 2 }).format(bps / 10000);
  const valid = isValidRate(rate);
  const unchanged = valid && Math.round(Number(rate) * 100) === currentBps;
  const canSubmit = Boolean(hostId) && valid && !unchanged && reasonCode !== '' && !busy;

  const submit = async () => {
    if (!hostId || !canSubmit) return;
    setBusy(true);
    setFormError(null);
    const result = await setCommissionRate(client, hostId, { rate: rate.trim(), reasonCode, note: note.trim() || null });
    setBusy(false);
    if (result.error || !result.data) {
      setFormError(errorMessage(result.error));
      return;
    }
    onClose();
    onSaved();
  };

  return {
    rate,
    setRate: (value: string) => {
      setFormError(null);
      setRate(value);
    },
    reasonCode,
    setReasonCode,
    note,
    setNote,
    busy,
    formError,
    canSubmit,
    unchanged,
    submit,
    currentText: percent(currentBps),
    reasonOptions: RATE_REASONS.map((value) => ({ value, label: translate(`reasons.${value}`) })),
    labels: {
      title: translate('rate.title'), description: translate('rate.description'), value: translate('rate.value'),
      valueHint: translate('rate.valueHint'), current: translate('rate.current'), unchanged: translate('rate.unchanged'),
      reason: translate('rate.reason'), chooseReason: translate('rate.chooseReason'), note: translate('rate.note'),
      noteHint: translate('rate.noteHint'), submit: translate('rate.submit'), cancel: translate('rate.cancel'),
    },
  };
};

export type CommissionRateDialogView = ReturnType<typeof useCommissionRateDialog>;
