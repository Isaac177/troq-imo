import { useCallback, useEffect, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { formatDateTime, formatDay } from '@mungini/utils';
import { useCityTimezone } from '@/hooks/cases/useCityTimezone';
import { closeCase, getCase } from '@/services/cases/cases.api';
import { OPEN_CASE_STATES, type CaseResolution, type HostCase } from '@/types/cases/cases';

const KNOWN_ERRORS = [
  'CASE_NOT_FOUND',
  'CASE_CLOSED',
  'CASE_VERSION_CONFLICT',
  'CASE_REASON_REQUIRED',
  'CASE_REASON_TOO_LONG',
  'FORBIDDEN',
];

const NAMESPACE = 'hostCases';

const textFromObject = (value: unknown): string | null => {
  if (typeof value === 'string') return value.trim() ? value.trim() : null;
  if (value && typeof value === 'object' && !Array.isArray(value)) {
    const record = value as Record<string, unknown>;
    const text = record.text ?? record.note ?? record.body ?? record.resolution;
    return typeof text === 'string' && text.trim() ? text : null;
  }
  return null;
};

const resolutionText = (value: CaseResolution): string | null => {
  if (typeof value !== 'string') return textFromObject(value);
  const trimmed = value.trim();
  if (!trimmed) return null;
  if (!trimmed.startsWith('"') && !trimmed.startsWith('{')) return trimmed;
  try {
    const parsed: unknown = JSON.parse(trimmed);
    return typeof parsed === 'string' ? parsed : textFromObject(parsed) ?? trimmed;
  } catch {
    return trimmed;
  }
};

export const useCaseDetail = (caseId: string | null, canWrite: boolean, onChanged: () => void) => {
  const { t } = useTranslation();
  const translateRaw = t as unknown as (key: string, options?: Record<string, unknown>) => string;
  const translate = useCallback((key: string, options?: Record<string, unknown>) => translateRaw(key, { ...options, ns: NAMESPACE }), [translateRaw]);
  const { client } = useSession();
  const timezone = useCityTimezone();
  const [detail, setDetail] = useState<HostCase | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');
  const [notice, setNotice] = useState('');
  const [closeOpen, setCloseOpen] = useState(false);
  const [resolution, setResolution] = useState('');
  const [busy, setBusy] = useState(false);
  const [formError, setFormError] = useState('');
  const [reloadToken, setReloadToken] = useState(0);

  const message = (code: string | null) => (code && KNOWN_ERRORS.includes(code) ? translate(`errors.${code}`) : translate('errors.generic'));

  useEffect(() => {
    setDetail(null);
    setError('');
    setNotice('');
    setCloseOpen(false);
    setResolution('');
    setFormError('');
  }, [caseId]);

  useEffect(() => {
    if (!caseId) return;
    let active = true;
    setLoading(true);
    setError('');
    void getCase(client, caseId).then((result) => {
      if (!active) return;
      setLoading(false);
      if (result.error || !result.data) {
        setError(message(result.error));
        return;
      }
      setDetail(result.data);
    }).catch(() => {
      if (!active) return;
      setLoading(false);
      setError(translate('error'));
    });
    return () => {
      active = false;
    };
  }, [client, caseId, reloadToken, translate]);

  const canAct = canWrite && OPEN_CASE_STATES.includes(detail?.state ?? '');

  const openClose = () => {
    setResolution('');
    setFormError('');
    setNotice('');
    setCloseOpen(true);
  };

  const dismissClose = () => {
    if (busy) return;
    setCloseOpen(false);
  };

  const submitClose = async () => {
    if (!detail || resolution.trim().length === 0) return;
    setBusy(true);
    setFormError('');
    const result = await closeCase(client, detail.id, detail.version, resolution.trim());
    setBusy(false);
    if (result.error || !result.data) {
      setFormError(message(result.error));
      return;
    }
    setCloseOpen(false);
    setResolution('');
    setNotice(translate('closed'));
    setReloadToken((value) => value + 1);
    onChanged();
  };

  return {
    detail,
    loading,
    error,
    notice,
    canAct,
    closeOpen,
    openClose,
    dismissClose,
    resolution,
    setResolution,
    canSubmitClose: resolution.trim().length > 0 && !busy,
    busy,
    formError,
    submitClose,
    resolutionText: detail ? resolutionText(detail.resolution) : null,
    openedText: detail ? formatDay(detail.createdAt, timezone) : '',
    deadlineText: detail ? formatDateTime(detail.deadlineAt, timezone) : '',
    stateLabel: (value: string) => translate(`states.${value}`),
    typeLabel: (value: string) => translate(`types.${value}`),
    labels: {
      close: translate('close'),
      closeCase: translate('closeCase'),
      closeTitle: translate('closeTitle'),
      closeBody: translate('closeBody'),
      resolutionLabel: translate('resolutionLabel'),
      resolutionPlaceholder: translate('resolutionPlaceholder'),
      confirmClose: translate('confirmClose'),
      dismiss: translate('dismiss'),
      closed: translate('closed'),
      error: translate('error'),
      loading: translate('detailLoading'),
      state: translate('state'),
      reservation: translate('reservation'),
      noReservation: translate('noReservation'),
      opened: translate('opened'),
      deadline: translate('deadline'),
      reason: translate('reason'),
      resolution: translate('resolution'),
    },
  };
};

export type CaseDetailView = ReturnType<typeof useCaseDetail>;
