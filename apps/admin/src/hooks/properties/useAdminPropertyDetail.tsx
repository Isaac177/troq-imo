import { useEffect, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { getProperty, listAmenities, reinstateProperty, suspendProperty } from '@/services/properties/properties.api';
import type { AdminPropertyDetail } from '@/types/properties/properties';

const KNOWN_ERRORS = ['PROPERTY_NOT_FOUND', 'LISTING_STATUS_INVALID', 'LISTING_VERSION_CONFLICT', 'REASON_CODE_REQUIRED', 'REASON_CODE_UNKNOWN', 'FORBIDDEN'];

export const useAdminPropertyDetail = (propertyId: string | null, onChanged: () => void) => {
  const { t, i18n } = useTranslation('adminProperties');
  const translate = t as unknown as (key: string) => string;
  const { client, city } = useSession();
  const [detail, setDetail] = useState<AdminPropertyDetail | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(false);
  const [notice, setNotice] = useState<string | null>(null);
  const [suspendOpen, setSuspendOpen] = useState(false);
  const [reason, setReason] = useState('');
  const [busy, setBusy] = useState(false);
  const [formError, setFormError] = useState<string | null>(null);
  const [amenityNames, setAmenityNames] = useState<Record<string, string>>({});

  const message = (code: string | null) => (code && KNOWN_ERRORS.includes(code) ? translate(`errors.${code}`) : translate('errors.generic'));

  useEffect(() => {
    setDetail(null);
    setNotice(null);
    setSuspendOpen(false);
    setFormError(null);
    if (!propertyId) return;
    let active = true;
    setLoading(true);
    setError(false);
    void getProperty(client, propertyId).then((result) => {
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
  }, [client, propertyId, city]);

  const submitSuspend = async () => {
    if (!detail) return;
    setBusy(true);
    setFormError(null);
    const result = await suspendProperty(client, detail.id, detail.version, reason);
    setBusy(false);
    if (result.error || !result.data) {
      setFormError(message(result.error));
      return;
    }
    setDetail(result.data);
    setSuspendOpen(false);
    setNotice(translate('saved'));
    onChanged();
  };

  const reinstate = async () => {
    if (!detail) return;
    setBusy(true);
    setFormError(null);
    const result = await reinstateProperty(client, detail.id, detail.version);
    setBusy(false);
    if (result.error || !result.data) {
      setFormError(message(result.error));
      return;
    }
    setDetail(result.data);
    setNotice(translate('saved'));
    onChanged();
  };

  useEffect(() => {
    let active = true;
    void listAmenities(client).then(({ data }) => {
      if (!active || !data) return;
      setAmenityNames(Object.fromEntries(data.data.items.map((item) => [item.code, item.name])));
    }).catch(() => undefined);
    return () => {
      active = false;
    };
  }, [client]);

  const statusLabel = (value: string) => {
    const key = `statuses.${value}`;
    const label = translate(key);
    return label === key ? value : label;
  };
  const amenities = detail ? (Array.isArray(detail.amenities) ? detail.amenities.map((code) => amenityNames[String(code)] ?? String(code)).join(', ') : '') : '';
  const canSuspend = Boolean(detail && ['APPROVED', 'PUBLISHED', 'PAUSED'].includes(detail.status));
  const canReinstate = detail?.status === 'SUSPENDED';

  return {
    detail, loading, error, notice, suspendOpen, setSuspendOpen, reason, setReason, busy, formError, submitSuspend, reinstate,
    canSuspend, canReinstate,
    name: detail ? (i18n.language.startsWith('fr') ? detail.nameFr : detail.nameEn) || detail.nameEn || detail.nameFr : '',
    statusLabel, amenities,
    reasonOptions: ['SAFETY_CONCERN', 'POLICY_VIOLATION', 'OPERATOR_REQUEST', 'OTHER'].map((value) => ({ value, label: translate(`reasons.${value}`) })),
    labels: {
      property: translate('property'), host: translate('host'), hostStatus: translate('hostStatus'), city: translate('city'),
      openRevisions: translate('openRevisions'), publishedRevision: translate('publishedRevision'), units: translate('units'),
      unitTypes: translate('unitTypes'), photos: translate('photos'), pendingRequests: translate('pendingRequests'), openReservations: translate('openReservations'),
      guestContact: translate('guestContact'), neighborhood: translate('neighborhood'), address: translate('address'), descriptionEn: translate('descriptionEn'),
      descriptionFr: translate('descriptionFr'), checkIn: translate('checkIn'), checkOut: translate('checkOut'), pets: translate('pets'),
      smoking: translate('smoking'), houseRulesEn: translate('houseRulesEn'), houseRulesFr: translate('houseRulesFr'), amenities: translate('amenities'),
      suspend: translate('suspend'), reinstate: translate('reinstate'), suspendTitle: translate('suspendTitle'), suspendBody: translate('suspendBody'),
      reason: translate('reason'), chooseReason: translate('chooseReason'), save: translate('save'), close: translate('close'),
      loading: translate('detailLoading'), error: translate('error'), status: translate('status'),
    },
  };
};

export type AdminPropertyDetailView = ReturnType<typeof useAdminPropertyDetail>;
