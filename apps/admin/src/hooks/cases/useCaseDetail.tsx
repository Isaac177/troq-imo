import { useEffect, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { formatCdf, formatDateTime } from '@mungini/utils';
import { closeCase, escalateCase, getCase, getEvidence, listCities } from '@/services/cases/cases.api';
import type { AdminCaseDetail, CaseEvidence, EvidenceContent } from '@/types/cases/cases';

const KNOWN_ERRORS = ['CASE_NOT_FOUND', 'CASE_CLOSED', 'CASE_ALREADY_ESCALATED', 'CASE_VERSION_CONFLICT', 'CASE_REASON_REQUIRED', 'CASE_REASON_TOO_LONG', 'EVIDENCE_NOT_FOUND', 'FORBIDDEN'];

export const useCaseDetail = (caseId: string | null, onChanged: () => void) => {
  const { t, i18n } = useTranslation('adminCases');
  const translate = t as unknown as (key: string) => string;
  const { client, city } = useSession();
  const [detail, setDetail] = useState<AdminCaseDetail | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(false);
  const [notice, setNotice] = useState<string | null>(null);
  const [action, setAction] = useState<'close' | 'escalate' | null>(null);
  const [text, setText] = useState('');
  const [busy, setBusy] = useState(false);
  const [formError, setFormError] = useState<string | null>(null);
  const [preview, setPreview] = useState<EvidenceContent | null>(null);
  const [previewError, setPreviewError] = useState<string | null>(null);
  const [reloadToken, setReloadToken] = useState(0);
  const [timezone, setTimezone] = useState('UTC');

  const message = (code: string | null) => (code && KNOWN_ERRORS.includes(code) ? translate(`errors.${code}`) : translate('errors.generic'));

  useEffect(() => {
    setDetail(null);
    setAction(null);
    setText('');
    setFormError(null);
    setNotice(null);
    setPreview(null);
    setPreviewError(null);
  }, [caseId]);

  useEffect(() => {
    if (!caseId) return;
    let active = true;
    setLoading(true);
    setError(false);
    void getCase(client, caseId).then((result) => {
      if (!active) return;
      setLoading(false);
      if (result.error || !result.data) {
        setError(true);
        return;
      }
      setDetail(result.data);
    });
    return () => {
      active = false;
    };
  }, [client, caseId, city, reloadToken]);

  useEffect(() => {
    let active = true;
    void listCities(client).then(({ data }) => {
      const match = data?.data.find((item) => item.code === city);
      if (active && match) setTimezone(match.timezone);
    }).catch(() => undefined);
    return () => {
      active = false;
    };
  }, [client, city]);

  const openAction = (next: 'close' | 'escalate') => {
    setAction(next);
    setText('');
    setFormError(null);
  };

  const submit = async () => {
    if (!detail || !action) return;
    setBusy(true);
    setFormError(null);
    const result = action === 'close'
      ? await closeCase(client, detail.item.id, detail.item.version, text)
      : await escalateCase(client, detail.item.id, detail.item.version, text);
    setBusy(false);
    if (result.error || !result.data) {
      setFormError(message(result.error));
      return;
    }
    setAction(null);
    setText('');
    setNotice(translate('saved'));
    setReloadToken((value) => value + 1);
    onChanged();
  };

  const openEvidence = async (evidence: CaseEvidence) => {
    setPreviewError(null);
    const result = await getEvidence(client, evidence.id);
    if (result.error || !result.data) {
      setPreviewError(message(result.error));
      return;
    }
    setPreview(result.data);
  };

  const canAct = detail ? detail.item.state === 'OPEN' || detail.item.state === 'AWAITING_RESPONSE' : false;
  const evidenceRows = (detail?.evidence ?? []).map((evidence) => ({ ...evidence, size: `${Math.max(1, Math.ceil(evidence.sizeBytes / 1024))} KB` }));

  return {
    detail,
    loading,
    error,
    notice,
    action,
    setAction,
    text,
    setText,
    busy,
    formError,
    submit,
    openAction,
    canAct,
    evidenceRows,
    openEvidence,
    preview,
    setPreview,
    previewError,
    time: (value: string | null | undefined) => (value ? formatDateTime(value, timezone) : ''),
    money: (value: string | null | undefined) => (value ? formatCdf(value, i18n.language) : '—'),
    stateLabel: (value: string) => translate(`states.${value}`),
    typeLabel: (value: string) => translate(`types.${value}`),
    labels: {
      operator: translate('operator'), reservation: translate('reservation'), deadline: translate('deadline'), createdAt: translate('createdAt'),
      claimAmount: translate('claimAmount'), reason: translate('reason'), evidence: translate('evidence'), noEvidence: translate('noEvidence'),
      escalated: translate('escalated'), escalationReason: translate('escalationReason'), resolution: translate('resolution'),
      closeCase: translate('closeCase'), escalateCase: translate('escalateCase'), closeTitle: translate('closeTitle'), closeBody: translate('closeBody'),
      escalateTitle: translate('escalateTitle'), escalateBody: translate('escalateBody'), resolutionPlaceholder: translate('resolutionPlaceholder'),
      escalationPlaceholder: translate('escalationPlaceholder'), confirmClose: translate('confirmClose'), confirmEscalate: translate('confirmEscalate'),
      dismiss: translate('dismiss'), saved: translate('saved'), loading: translate('detailLoading'), error: translate('error'),
      previewTitle: translate('previewTitle'), previewUnavailable: translate('previewUnavailable'), close: translate('close'), state: translate('state'),
    },
  };
};

export type CaseDetailView = ReturnType<typeof useCaseDetail>;
