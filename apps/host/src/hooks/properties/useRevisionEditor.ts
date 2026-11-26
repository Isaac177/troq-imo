import { useEffect, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { listAmenities, submitRevision, updateRevision } from '@/services/properties/properties.api';
import type { Amenity, ListingContent, Revision } from '@/services/properties/properties.api';

const KNOWN_ERRORS = ['REVISION_NOT_FOUND', 'REVISION_ALREADY_OPEN', 'LISTING_NOT_READY', 'LISTING_STATUS_INVALID', 'LISTING_VERSION_CONFLICT', 'LISTING_ARCHIVED', 'CHECK_IN_WINDOW_INVALID', 'FORBIDDEN'];

export const useRevisionEditor = (propertyId: string | null, revision: Revision | null, onSaved: (next: Revision) => void, onSubmitted: () => void) => {
  const { t } = useTranslation('host');
  const translate = t as unknown as (key: string) => string;
  const { client } = useSession();
  const [nameEn, setNameEn] = useState('');
  const [nameFr, setNameFr] = useState('');
  const [descriptionEn, setDescriptionEn] = useState('');
  const [descriptionFr, setDescriptionFr] = useState('');
  const [neighborhood, setNeighborhood] = useState('');
  const [addressLine, setAddressLine] = useState('');
  const [latitude, setLatitude] = useState('');
  const [longitude, setLongitude] = useState('');
  const [checkInFrom, setCheckInFrom] = useState('');
  const [checkInUntil, setCheckInUntil] = useState('');
  const [checkOutBy, setCheckOutBy] = useState('');
  const [houseRulesEn, setHouseRulesEn] = useState('');
  const [houseRulesFr, setHouseRulesFr] = useState('');
  const [petsPolicy, setPetsPolicy] = useState('NOT_ALLOWED');
  const [smokingPolicy, setSmokingPolicy] = useState('NOT_ALLOWED');
  const [guestContactPhone, setGuestContactPhone] = useState('');
  const [guestContactEmail, setGuestContactEmail] = useState('');
  const [amenities, setAmenities] = useState<string[]>([]);
  const [catalog, setCatalog] = useState<Amenity[]>([]);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [blockers, setBlockers] = useState<string[]>([]);
  const [notice, setNotice] = useState<string | null>(null);

  const message = (code: string | null) => (code && KNOWN_ERRORS.includes(code) ? translate(`revisionErrors.${code}`) : translate('revisionErrors.generic'));

  useEffect(() => {
    let active = true;
    void listAmenities(client).then((result) => {
      if (active) setCatalog(result.data ?? []);
    });
    return () => {
      active = false;
    };
  }, [client]);

  useEffect(() => {
    setError(null);
    setBlockers([]);
    setNotice(null);
    if (!revision) return;
    const content = revision.content;
    setNameEn(content.nameEn);
    setNameFr(content.nameFr);
    setDescriptionEn(content.descriptionEn);
    setDescriptionFr(content.descriptionFr);
    setNeighborhood(content.neighborhood);
    setAddressLine(content.addressLine);
    setLatitude(content.latitude === null || content.latitude === undefined ? '' : String(content.latitude));
    setLongitude(content.longitude === null || content.longitude === undefined ? '' : String(content.longitude));
    setCheckInFrom(content.checkInFrom ?? '');
    setCheckInUntil(content.checkInUntil ?? '');
    setCheckOutBy(content.checkOutBy ?? '');
    setHouseRulesEn(content.houseRulesEn);
    setHouseRulesFr(content.houseRulesFr);
    setPetsPolicy(content.petsPolicy ?? 'NOT_ALLOWED');
    setSmokingPolicy(content.smokingPolicy ?? 'NOT_ALLOWED');
    setGuestContactPhone(content.guestContactPhone);
    setGuestContactEmail(content.guestContactEmail);
    setAmenities(content.amenities);
  }, [revision]);

  const content = (): ListingContent => ({
    nameEn, nameFr, descriptionEn, descriptionFr, neighborhood, addressLine,
    latitude: latitude.trim() ? Number(latitude) : null,
    longitude: longitude.trim() ? Number(longitude) : null,
    checkInFrom: checkInFrom.trim() || null,
    checkInUntil: checkInUntil.trim() || null,
    checkOutBy: checkOutBy.trim() || null,
    houseRulesEn, houseRulesFr,
    petsPolicy: petsPolicy || null,
    smokingPolicy: smokingPolicy || null,
    guestContactPhone, guestContactEmail,
    amenities,
  });

  const save = async (): Promise<Revision | null> => {
    if (!propertyId || !revision) return null;
    const result = await updateRevision(client, propertyId, revision.id, revision.version, content());
    if (result.error || !result.data) {
      setError(message(result.error));
      setBlockers(result.blockers);
      return null;
    }
    onSaved(result.data);
    return result.data;
  };

  const saveDraft = async () => {
    setBusy(true);
    setError(null);
    setBlockers([]);
    const saved = await save();
    setBusy(false);
    if (saved) setNotice(translate('revisionSaved'));
  };

  const saveAndSubmit = async () => {
    if (!propertyId || !revision) return;
    setBusy(true);
    setError(null);
    setBlockers([]);
    const saved = await save();
    if (!saved) {
      setBusy(false);
      return;
    }
    const result = await submitRevision(client, propertyId, saved.id, saved.version);
    setBusy(false);
    if (result.error || !result.data) {
      setError(message(result.error));
      setBlockers(result.blockers);
      return;
    }
    setNotice(translate('revisionSubmitted'));
    onSubmitted();
  };

  const toggleAmenity = (code: string) => setAmenities((current) => (current.includes(code) ? current.filter((value) => value !== code) : [...current, code]));

  return {
    nameEn, setNameEn, nameFr, setNameFr, descriptionEn, setDescriptionEn, descriptionFr, setDescriptionFr, neighborhood, setNeighborhood,
    addressLine, setAddressLine, latitude, setLatitude, longitude, setLongitude, checkInFrom, setCheckInFrom, checkInUntil, setCheckInUntil,
    checkOutBy, setCheckOutBy, houseRulesEn, setHouseRulesEn, houseRulesFr, setHouseRulesFr, petsPolicy, setPetsPolicy, smokingPolicy, setSmokingPolicy,
    guestContactPhone, setGuestContactPhone, guestContactEmail, setGuestContactEmail, amenities, toggleAmenity, catalog,
    busy, error, blockers, notice, saveDraft, saveAndSubmit,
    blockerLabels: blockers.map((code) => {
      const key = `blockerLabels.${code}`;
      const label = translate(key);
      return label === key ? code : label;
    }),
    petsOptions: ['NOT_ALLOWED', 'ALLOWED', 'ON_REQUEST'].map((value) => ({ value, label: translate(`policies.${value}`) })),
    smokingOptions: ['NOT_ALLOWED', 'ALLOWED'].map((value) => ({ value, label: translate(`policies.${value}`) })),
    labels: {
      saveDraft: translate('revisionSaveDraft'), saveAndSubmit: translate('saveAndSubmit'), blockers: translate('revisionBlockers'),
      close: translate('close'), nameEn: translate('revisionFields.nameEn'), nameFr: translate('revisionFields.nameFr'),
      descriptionEn: translate('revisionFields.descriptionEn'), descriptionFr: translate('revisionFields.descriptionFr'),
      neighborhood: translate('revisionFields.neighborhood'), addressLine: translate('revisionFields.addressLine'),
      latitude: translate('revisionFields.latitude'), longitude: translate('revisionFields.longitude'),
      checkInFrom: translate('revisionFields.checkInFrom'), checkInUntil: translate('revisionFields.checkInUntil'),
      checkOutBy: translate('revisionFields.checkOutBy'), houseRulesEn: translate('revisionFields.houseRulesEn'),
      houseRulesFr: translate('revisionFields.houseRulesFr'), petsPolicy: translate('revisionFields.petsPolicy'),
      smokingPolicy: translate('revisionFields.smokingPolicy'), guestContactPhone: translate('revisionFields.guestContactPhone'),
      guestContactEmail: translate('revisionFields.guestContactEmail'), amenities: translate('revisionFields.amenities'),
    },
  };
};

export type RevisionEditorView = ReturnType<typeof useRevisionEditor>;
