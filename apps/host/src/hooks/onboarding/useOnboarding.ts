import { useEffect, useMemo, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { hostApi, type ApplicationView } from '@/services/host/host.api';

const PAGE_SIZE = 10;

export const useOnboarding = () => {
  const { t } = useTranslation('host');
  const translate = t as unknown as (key: string) => string;
  const { client } = useSession();
  const api = useMemo(() => hostApi(client), [client]);
  const [application, setApplication] = useState<ApplicationView | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [displayName, setDisplayName] = useState('');
  const [kind, setKind] = useState('INDIVIDUAL');
  const [documentKinds, setDocumentKinds] = useState<string[]>([]);
  const [documentKind, setDocumentKind] = useState('');
  const [file, setFile] = useState<{ name: string; mimeType: string; base64: string } | null>(null);

  const refresh = async () => {
    const result = await api.myApplications(1, PAGE_SIZE);
    if (result.error) {
      setError(t('error'));
      setLoading(false);
      return;
    }
    const items = result.data?.items ?? [];
    setApplication(items[0] ?? null);
    setLoading(false);
  };

  useEffect(() => {
    void refresh();
  }, [api]);

  useEffect(() => {
    void client.api.GET('/api/public/host-reference').then(({ data }) => {
      if (!data) return;
      setDocumentKinds(data.data.documentKinds);
    });
  }, [client]);

  const readFile = (input: File | null) => {
    if (!input) {
      setFile(null);
      return;
    }
    const reader = new FileReader();
    reader.onload = () => {
      const value = String(reader.result ?? '');
      const base64 = value.includes(',') ? value.slice(value.indexOf(',') + 1) : value;
      setFile({ name: input.name, mimeType: input.type, base64 });
    };
    reader.readAsDataURL(input);
  };

  const saveDraft = async () => {
    const result = await api.saveApplication(displayName.trim(), kind);
    if (result.error) {
      setError(t('error'));
      return;
    }
    setApplication(result.data);
  };

  const addDocument = async () => {
    if (!application || !file || !documentKind) return;
    const result = await api.addDocument(application.id, {
      kind: documentKind,
      fileName: file.name,
      mimeType: file.mimeType,
      contentBase64: file.base64,
    });
    if (result.error) {
      setError(t('error'));
      return;
    }
    setFile(null);
    setDocumentKind('');
    await refresh();
  };

  const removeDocument = async (documentId: string) => {
    if (!application) return;
    const result = await api.removeDocument(application.id, documentId);
    if (result.error) {
      setError(t('error'));
      return;
    }
    await refresh();
  };

  const submit = async () => {
    if (!application) return;
    const result = await api.submitApplication(application.id);
    if (result.error) {
      setError(t('error'));
      return;
    }
    setApplication(result.data);
  };

  const withdraw = async () => {
    if (!application) return;
    const result = await api.withdrawApplication(application.id);
    if (result.error) {
      setError(t('error'));
      return;
    }
    setApplication(result.data);
  };

  const editable = !application || application.status === 'DRAFT';

  return {
    application,
    loading,
    error,
    displayName,
    setDisplayName,
    kind,
    setKind,
    editable,
    documentKinds,
    documentKind,
    setDocumentKind,
    readFile,
    file,
    addDocument,
    removeDocument,
    saveDraft,
    submit,
    withdraw,
    labels: {
      title: t('onboardingTitle'),
      subtitle: t('onboardingSubtitle'),
      displayName: t('displayName'),
      kind: t('kind'),
      individual: t('kindIndividual'),
      business: t('kindBusiness'),
      documents: t('documents'),
      documentKind: t('documentKind'),
      documentFile: t('documentFile'),
      addDocument: t('addDocument'),
      submit: t('submitApplication'),
      withdraw: t('withdrawApplication'),
      saveDraft: t('saveDraft'),
      noApplication: t('noApplication'),
      start: t('startApplication'),
      status: (status: string) => {
        const map: Record<string, string> = {
          DRAFT: t('applicationDraft'),
          PENDING_REVIEW: t('applicationPending'),
          APPROVED: t('applicationApproved'),
          REJECTED: t('applicationRejected'),
          WITHDRAWN: t('applicationWithdrawn'),
        };
        return map[status] ?? status;
      },
      documentKindLabel: (kindValue: string) => translate(`documentKinds.${kindValue}`),
      remove: t('removeMember'),
      empty: t('empty'),
    },
  };
};
