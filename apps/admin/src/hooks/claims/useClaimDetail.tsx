import { useEffect, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { formatCdf, formatDateTime } from '@mungini/utils';
import { decideClaim, getClaim, recordPayout } from '@/services/claims/claims.api';
import type { ClaimDetail } from '@/types/claims/claims';

const KNOWN_ERRORS = ['CLAIM_NOT_FOUND', 'CLAIM_STATUS_UNKNOWN', 'CLAIM_REASON_UNKNOWN', 'CLAIM_PARTIAL_AMOUNT_INVALID', 'CLAIM_ALREADY_DECIDED', 'CLAIM_VERSION_CONFLICT', 'CLAIM_DECISION_NOT_FOUND', 'PAYOUT_STATE_UNKNOWN', 'PAYOUT_REF_REQUIRED', 'PAYOUT_REF_TOO_LONG', 'PAYOUT_NOT_DUE', 'PAYOUT_ALREADY_PAID', 'FORBIDDEN'];
const REASONS: Record<string, string[]> = {
  APPROVED: ['DAMAGE_CONFIRMED', 'EVIDENCE_SUFFICIENT'],
  PARTIAL: ['PARTIAL_EVIDENCE', 'WEAR_AND_TEAR', 'AMOUNT_ADJUSTED'],
  REJECTED: ['NO_EVIDENCE', 'PRE_EXISTING_DAMAGE', 'GUEST_DISPUTE_UPHELD'],
};

export const useClaimDetail = (caseId: string | null, onChanged: () => void) => {
  const { t, i18n } = useTranslation('adminClaims');
  const translate = t as unknown as (key: string) => string;
  const { client, city } = useSession();
  const [detail, setDetail] = useState<ClaimDetail | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(false);
  const [notice, setNotice] = useState<string | null>(null);
  const [decisionOpen, setDecisionOpen] = useState(false);
  const [decision, setDecision] = useState('APPROVED');
  const [reasonCode, setReasonCode] = useState('DAMAGE_CONFIRMED');
  const [approvedAmount, setApprovedAmount] = useState('');
  const [payoutOpen, setPayoutOpen] = useState(false);
  const [payoutState, setPayoutState] = useState('PENDING');
  const [payoutRef, setPayoutRef] = useState('');
  const [busy, setBusy] = useState(false);
  const [formError, setFormError] = useState<string | null>(null);
  const [timezone, setTimezone] = useState('UTC');

  const message = (code: string | null) => (code && KNOWN_ERRORS.includes(code) ? translate(`errors.${code}`) : translate('errors.generic'));

  useEffect(() => {
    setDetail(null);
    setNotice(null);
    setDecisionOpen(false);
    setPayoutOpen(false);
    setFormError(null);
    if (!caseId) return;
    let active = true;
    setLoading(true);
    setError(false);
    void getClaim(client, caseId).then((result) => {
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
  }, [client, caseId, city]);

  useEffect(() => {
    let active = true;
    void client.api.GET('/api/public/cities').then(({ data }) => {
      const match = data?.data.find((item) => item.code === city);
      if (active && match) setTimezone(match.timezone);
    }).catch(() => undefined);
    return () => {
      active = false;
    };
  }, [client, city]);

  const openDecision = () => {
    setDecision('APPROVED');
    setReasonCode('DAMAGE_CONFIRMED');
    setApprovedAmount('');
    setFormError(null);
    setDecisionOpen(true);
  };
  const changeDecision = (next: string) => {
    setDecision(next);
    setReasonCode(REASONS[next]?.[0] ?? '');
  };
  const submitDecision = async () => {
    if (!detail) return;
    setBusy(true);
    setFormError(null);
    const result = await decideClaim(client, detail.caseId, {
      decision,
      reasonCode,
      approvedAmount: decision === 'PARTIAL' ? approvedAmount : null,
      expectedVersion: detail.version,
    });
    setBusy(false);
    if (result.error || !result.data) {
      setFormError(message(result.error));
      return;
    }
    setDetail(result.data);
    setDecisionOpen(false);
    setNotice(translate('saved'));
    onChanged();
  };

  const openPayout = () => {
    setPayoutState('PENDING');
    setPayoutRef('');
    setFormError(null);
    setPayoutOpen(true);
  };
  const submitPayout = async () => {
    if (!detail) return;
    setBusy(true);
    setFormError(null);
    const result = await recordPayout(client, detail.caseId, { payoutState, payoutRef: payoutRef.trim() || null });
    if (result.error) {
      setBusy(false);
      setFormError(message(result.error));
      return;
    }
    const refreshed = await getClaim(client, detail.caseId);
    setBusy(false);
    if (refreshed.data) setDetail(refreshed.data);
    setPayoutOpen(false);
    setNotice(translate('saved'));
    onChanged();
  };

  return {
    detail, loading, error, notice, decisionOpen, setDecisionOpen, decision, changeDecision, reasonCode, setReasonCode, approvedAmount, setApprovedAmount,
    payoutOpen, setPayoutOpen, payoutState, setPayoutState, payoutRef, setPayoutRef, busy, formError, openDecision, submitDecision, openPayout, submitPayout,
    canDecide: Boolean(detail && !detail.decision),
    canPayout: Boolean(detail?.decision && detail.decision.payoutState !== 'PAID' && Number(detail.decision.approvedAmount) > 0),
    money: (value: string | null | undefined) => (value ? formatCdf(value, i18n.language) : '—'),
    time: (value: string | null | undefined) => (value ? formatDateTime(value, timezone) : ''),
    propertyName: detail ? (i18n.language.startsWith('fr') ? detail.propertyNameFr : detail.propertyNameEn) || detail.propertyNameEn || detail.propertyNameFr || '' : '',
    decisionLabel: (value: string) => translate(`decisions.${value}`),
    reasonLabel: (value: string) => {
      const key = `reasonCodes.${value}`;
      const label = translate(key);
      return label === key ? value : label;
    },
    payoutLabel: (value: string) => translate(`payoutStates.${value}`),
    decisionOptions: ['APPROVED', 'PARTIAL', 'REJECTED'].map((value) => ({ value, label: translate(`decisions.${value}`) })),
    reasonOptions: (REASONS[decision] ?? []).map((value) => ({ value, label: translate(`reasonCodes.${value}`) })),
    payoutOptions: ['PENDING', 'PAID', 'FAILED'].map((value) => ({ value, label: translate(`payoutStates.${value}`) })),
    labels: {
      property: translate('property'), host: translate('host'), guest: translate('guest'), reference: translate('reference'),
      claimAmount: translate('claimAmount'), approvedAmount: translate('approvedAmount'), decision: translate('decision'), payout: translate('payout'),
      deadline: translate('deadline'), stayTotal: translate('stayTotal'), checkIn: translate('checkIn'), checkOut: translate('checkOut'),
      openedAt: translate('openedAt'), reason: translate('reason'), evidence: translate('evidence'), noEvidence: translate('noEvidence'),
      decide: translate('decide'), decisionTitle: translate('decisionTitle'), decisionBody: translate('decisionBody'), partialAmount: translate('partialAmount'),
      reasons: translate('reasons'), chooseReason: translate('chooseReason'), payoutTitle: translate('payoutTitle'), payoutBody: translate('payoutBody'),
      payoutState: translate('payoutState'), payoutRef: translate('payoutRef'), payoutRefHint: translate('payoutRefHint'), save: translate('save'),
      close: translate('close'), dismiss: translate('dismiss'), loading: translate('detailLoading'), error: translate('error'),
    },
  };
};

export type ClaimDetailView = ReturnType<typeof useClaimDetail>;
