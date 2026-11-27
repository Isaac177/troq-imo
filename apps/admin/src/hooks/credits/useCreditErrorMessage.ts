import { useTranslation } from '@mungini/i18n';

const KNOWN_ERRORS = [
  'HOST_NOT_FOUND',
  'AMOUNT_INVALID',
  'TOPUP_REASON_UNKNOWN',
  'ADJUST_REASON_UNKNOWN',
  'DIRECTION_UNKNOWN',
  'PAYMENT_REF_REQUIRED',
  'PAYMENT_REF_TOO_LONG',
  'NOTE_TOO_LONG',
  'RATE_INVALID',
  'RATE_REASON_UNKNOWN',
  'RATE_UNCHANGED',
  'PERMISSION_MISSING',
];

export const useCreditErrorMessage = () => {
  const { t } = useTranslation('adminCredits');
  const translate = t as unknown as (key: string) => string;
  return (code: string | null) => (code && KNOWN_ERRORS.includes(code) ? translate(`errors.${code}`) : translate('errors.generic'));
};
