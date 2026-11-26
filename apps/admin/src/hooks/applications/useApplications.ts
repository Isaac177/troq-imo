import { useEffect, useMemo, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { useDebouncedValue } from '@mungini/ui-web';
import { adminApi, type AdminApplicationView, type ApplicationView, type HostReferenceView } from '@/services/admin/admin.api';

const PAGE_SIZE = 20;

export const useApplications = () => {
  const { t } = useTranslation('admin');
  const translate = t as unknown as (key: string) => string;
  const { client } = useSession();
  const api = useMemo(() => adminApi(client), [client]);
  const [status, setStatus] = useState('');
  const [search, setSearch] = useState('');
  const term = useDebouncedValue(search, 250);
  const [page, setPage] = useState(1);
  const [items, setItems] = useState<AdminApplicationView[]>([]);
  const [total, setTotal] = useState(0);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [selected, setSelected] = useState<ApplicationView | null>(null);
  const [reference, setReference] = useState<HostReferenceView | null>(null);
  const [decision, setDecision] = useState('');
  const [reason, setReason] = useState('');
  const [docReason, setDocReason] = useState('');
  const [busy, setBusy] = useState(false);

  useEffect(() => {
    setPage(1);
  }, [status, term]);

  useEffect(() => {
    let active = true;
    setLoading(true);
    setError(null);
    void api.applications(status, term, page, PAGE_SIZE).then((result) => {
      if (!active) return;
      setLoading(false);
      if (result.error) {
        setError(t('error'));
        return;
      }
      setItems(result.data?.items ?? []);
      setTotal(result.data?.total ?? 0);
    });
    return () => {
      active = false;
    };
  }, [api, status, term, page, t]);

  useEffect(() => {
    void api.reference().then((result) => {
      if (result.data) setReference(result.data);
    });
  }, [api]);

  const open = async (row: AdminApplicationView) => {
    setError(null);
    setDecision('');
    setReason('');
    const result = await api.application(row.id);
    if (result.error) {
      setError(t('error'));
      return;
    }
    setSelected(result.data);
  };

  const close = () => setSelected(null);

  const decide = async () => {
    if (!selected || !decision || !reason) return;
    setBusy(true);
    const result = await api.decide(selected.id, decision, reason);
    setBusy(false);
    if (result.error) {
      setError(t('error'));
      return;
    }
    setSelected(result.data);
    const refreshed = await api.applications(status, term, page, PAGE_SIZE);
    setItems(refreshed.data?.items ?? []);
    setTotal(refreshed.data?.total ?? 0);
  };

  const reviewDocument = async (documentId: string, next: string) => {
    if (!selected) return;
    const result = await api.reviewDocument(selected.id, documentId, next, next === 'REJECTED' ? docReason : null);
    if (result.error) {
      setError(t('error'));
      return;
    }
    setDocReason('');
    const refreshed = await api.application(selected.id);
    setSelected(refreshed.data);
  };

  const reasons = decision === 'APPROVE' ? reference?.approveReasons ?? [] : decision === 'REJECT' ? reference?.rejectReasons ?? [] : [];

  return {
    status,
    setStatus,
    search,
    setSearch,
    page,
    setPage,
    pageSize: PAGE_SIZE,
    items,
    total,
    loading,
    error,
    selected,
    open,
    close,
    decision,
    setDecision,
    reason,
    setReason,
    reasons,
    decide,
    busy,
    reviewDocument,
    docReason,
    setDocReason,
    documentReasons: reference?.documentRejectReasons ?? [],
    labels: {
      title: t('applicationsTitle'),
      subtitle: t('applicationsSubtitle'),
      application: t('application'),
      status: t('status'),
      submitted: t('submitted'),
      search: t('search'),
      all: t('allStatuses'),
      decide: t('decide'),
      approve: t('approve'),
      reject: t('reject'),
      reason: t('reason'),
      chooseReason: t('chooseReason'),
      documents: t('documents'),
      verify: t('verify'),
      rejectDocument: t('rejectDocument'),
      documentKind: t('documentKind'),
      fileName: t('fileName'),
      noDocuments: t('noDocuments'),
      empty: t('applicationsEmpty'),
      error: t('error'),
      statusLabel: (value: string) => translate(`applicationStatuses.${value}`),
      documentStatus: (value: string) => translate(`documentStatuses.${value}`),
      documentKindLabel: (value: string) => translate(`documentKinds.${value}`),
      reasonLabel: (value: string) => translate(`reasonLabels.${value}`),
    },
  };
};
