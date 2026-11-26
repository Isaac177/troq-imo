import { useEffect, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { formatDateTime } from '@mungini/utils';
import { createRevision, getRevision, listCities, listRevisions, withdrawRevision } from '@/services/properties/properties.api';
import type { Revision, RevisionSummary } from '@/services/properties/properties.api';

const KNOWN_ERRORS = ['REVISION_NOT_FOUND', 'REVISION_ALREADY_OPEN', 'LISTING_STATUS_INVALID', 'LISTING_VERSION_CONFLICT', 'LISTING_ARCHIVED', 'LISTING_NOT_READY', 'FORBIDDEN'];

export const usePropertyRevisions = (propertyId: string | null, onChanged: () => void) => {
  const { t } = useTranslation('host');
  const translate = t as unknown as (key: string) => string;
  const { client, city } = useSession();
  const [rows, setRows] = useState<RevisionSummary[]>([]);
  const [total, setTotal] = useState(0);
  const [page, setPage] = useState(1);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [notice, setNotice] = useState<string | null>(null);
  const [editing, setEditing] = useState<Revision | null>(null);
  const [busy, setBusy] = useState(false);
  const [withdrawTarget, setWithdrawTarget] = useState<RevisionSummary | null>(null);
  const [refreshToken, setRefreshToken] = useState(0);
  const [timezone, setTimezone] = useState('UTC');

  const message = (code: string | null) => (code && KNOWN_ERRORS.includes(code) ? translate(`revisionErrors.${code}`) : translate('revisionErrors.generic'));

  useEffect(() => {
    setPage(1);
    setEditing(null);
    setNotice(null);
    setError(null);
  }, [propertyId]);

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

  useEffect(() => {
    if (!propertyId) return;
    let active = true;
    setLoading(true);
    setError(null);
    void listRevisions(client, propertyId, page).then((result) => {
      if (!active) return;
      setLoading(false);
      if (result.error || !result.data) {
        setError(message(result.error));
        return;
      }
      setRows(result.data.items);
      setTotal(result.data.total);
    });
    return () => {
      active = false;
    };
  }, [client, propertyId, page, refreshToken, translate]);

  const reload = () => setRefreshToken((value) => value + 1);
  const createNew = async () => {
    if (!propertyId) return;
    setBusy(true);
    setError(null);
    const result = await createRevision(client, propertyId);
    setBusy(false);
    if (result.error || !result.data) {
      setError(message(result.error));
      return;
    }
    setEditing(result.data);
    reload();
    onChanged();
  };

  const open = async (revisionId: string) => {
    if (!propertyId) return;
    setBusy(true);
    setError(null);
    const result = await getRevision(client, propertyId, revisionId);
    setBusy(false);
    if (result.error || !result.data) {
      setError(message(result.error));
      return;
    }
    setEditing(result.data);
  };

  const confirmWithdraw = async () => {
    if (!propertyId || !withdrawTarget) return;
    setBusy(true);
    const result = await withdrawRevision(client, propertyId, withdrawTarget.id, withdrawTarget.version);
    setBusy(false);
    if (result.error) {
      setError(message(result.error));
      setWithdrawTarget(null);
      return;
    }
    setWithdrawTarget(null);
    setNotice(translate('revisionWithdrawn'));
    reload();
    onChanged();
  };

  return {
    rows, total, page, setPage, loading, error, notice, busy, editing, setEditing, createNew, open,
    withdrawTarget, setWithdrawTarget, confirmWithdraw, reload,
    statusLabel: (status: string) => {
      const key = `revisionStatuses.${status}`;
      const label = translate(key);
      return label === key ? status : label;
    },
    canWithdraw: (status: string) => status === 'SUBMITTED',
    canEdit: (status: string) => status === 'DRAFT' || status === 'CHANGES_REQUESTED',
    time: (value: string | null | undefined) => (value ? formatDateTime(value, timezone) : ''),
    labels: {
      revisions: translate('revisions'), newRevision: translate('newRevision'), noRevisions: translate('noRevisions'),
      openRevision: translate('openRevision'), withdrawRevision: translate('withdrawRevision'),
      confirmWithdrawTitle: translate('confirmWithdrawRevisionTitle'), confirmWithdrawBody: translate('confirmWithdrawRevisionBody'),
      dismiss: translate('close'), error: translate('revisionErrors.generic'),
    },
  };
};

export type PropertyRevisionsView = ReturnType<typeof usePropertyRevisions>;
