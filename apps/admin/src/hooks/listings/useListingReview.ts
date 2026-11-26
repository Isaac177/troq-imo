import { useEffect, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { decideListing, getRevision } from '@/services/listings/listings.api';
import type { AdminRevision, ListingContent } from '@/types/listings/listings';

const KNOWN_ERRORS = ['REASON_CODES_REQUIRED', 'REASON_CODES_NOT_ALLOWED', 'REASON_CODE_UNKNOWN', 'DECISION_UNKNOWN', 'REVISION_NOT_FOUND', 'PROPERTY_NOT_FOUND', 'LISTING_NOT_READY', 'FORBIDDEN'];
const FIELD_KEYS: (keyof ListingContent | 'coordinates')[] = ['nameEn', 'nameFr', 'descriptionEn', 'descriptionFr', 'neighborhood', 'addressLine', 'coordinates', 'checkInFrom', 'checkInUntil', 'checkOutBy', 'houseRulesEn', 'houseRulesFr', 'petsPolicy', 'smokingPolicy', 'guestContactPhone', 'guestContactEmail', 'amenities'];
const REASON_CODES = ['CONTENT_INCOMPLETE', 'MEDIA_UNSUITABLE', 'DESCRIPTION_MISLEADING', 'LOCATION_UNCLEAR', 'POLICY_MISSING', 'OTHER'];

const asText = (content: ListingContent, key: keyof ListingContent | 'coordinates'): string => {
  if (key === 'coordinates') {
    return content.latitude !== null && content.latitude !== undefined && content.longitude !== null && content.longitude !== undefined ? `${content.latitude}, ${content.longitude}` : '';
  }
  const value = content[key];
  if (Array.isArray(value)) return value.join(', ');
  return value === null || value === undefined ? '' : String(value);
};

export const useListingReview = (revisionId: string | null, onChanged: () => void, onDecided: () => void) => {
  const { t } = useTranslation('adminListings');
  const translate = t as unknown as (key: string) => string;
  const { client, city } = useSession();
  const [detail, setDetail] = useState<AdminRevision | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [decision, setDecision] = useState<string | null>(null);
  const [reasonCodes, setReasonCodes] = useState<string[]>([]);
  const [guidance, setGuidance] = useState('');
  const [busy, setBusy] = useState(false);
  const [formError, setFormError] = useState<string | null>(null);
  const [blockers, setBlockers] = useState<string[]>([]);

  const message = (code: string | null) => (code && KNOWN_ERRORS.includes(code) ? translate(`errors.${code}`) : translate('errors.generic'));

  useEffect(() => {
    setDetail(null);
    setDecision(null);
    setReasonCodes([]);
    setGuidance('');
    setFormError(null);
    setBlockers([]);
  }, [revisionId]);

  useEffect(() => {
    if (!revisionId) return;
    let active = true;
    setLoading(true);
    setError(null);
    void getRevision(client, revisionId).then((result) => {
      if (!active) return;
      setLoading(false);
      if (result.error || !result.data) {
        setError(translate('error'));
        return;
      }
      setDetail(result.data);
    });
    return () => {
      active = false;
    };
  }, [client, city, revisionId, translate]);

  const openDecision = (next: 'APPROVE' | 'REQUEST_CHANGES' | 'REJECT') => {
    setDecision(next);
    setReasonCodes([]);
    setGuidance('');
    setFormError(null);
    setBlockers([]);
  };
  const toggleReason = (code: string) => setReasonCodes((current) => (current.includes(code) ? current.filter((value) => value !== code) : [...current, code]));
  const submit = async () => {
    if (!detail || !decision) return;
    if (decision !== 'APPROVE' && reasonCodes.length === 0) {
      setFormError(translate('errors.REASON_CODES_REQUIRED'));
      return;
    }
    setBusy(true);
    setFormError(null);
    setBlockers([]);
    const result = await decideListing(client, detail.revision.id, {
      decision,
      reasonCodes: decision === 'APPROVE' ? [] : reasonCodes,
      guidance: guidance.trim() || null,
      expectedVersion: detail.revision.version,
    });
    setBusy(false);
    if (result.error || !result.data) {
      setFormError(message(result.error));
      setBlockers(result.blockers);
      return;
    }
    setDecision(null);
    onChanged();
    onDecided();
  };

  const fields = detail ? FIELD_KEYS.map((key) => {
    const live = asText(detail.live, key);
    const proposed = asText(detail.revision.content, key);
    return { key, label: translate(`fields.${key}`), live, proposed, changed: live !== proposed };
  }) : [];

  return {
    detail, loading, error, decision, setDecision, openDecision, reasonCodes, toggleReason, guidance, setGuidance, busy, formError, blockers, submit,
    fields,
    blockerLabels: blockers.map((code) => {
      const key = `blockerLabels.${code}`;
      const label = translate(key);
      return label === key ? code : label;
    }),
    reasonOptions: REASON_CODES.map((value) => ({ value, label: translate(`reasonCodes.${value}`) })),
    reviewStatus: detail ? translate(`statuses.${detail.revision.reviewStatus}`) : '',
    labels: {
      live: translate('live'), proposed: translate('proposed'), review: translate('review'), decision: translate('decision'),
      approve: translate('approve'), requestChanges: translate('requestChanges'), reject: translate('reject'), reasons: translate('reasons'),
      guidance: translate('guidance'), guidancePlaceholder: translate('guidancePlaceholder'), save: translate('save'), close: translate('close'),
      loading: translate('loading'), error: translate('error'), blockers: translate('blockers'), changed: translate('changed'), unchanged: translate('unchanged'),
    },
  };
};

export type ListingReviewView = ReturnType<typeof useListingReview>;
