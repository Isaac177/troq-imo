import { useCallback, useEffect, useRef, useState, type ChangeEvent } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { formatDateTime } from '@mungini/utils';
import { useCityTimezone } from '@/hooks/cases/useCityTimezone';
import { addEvidence, getEvidence, listEvidence } from '@/services/cases/cases.api';
import { EVIDENCE_MIME_TYPES, MAX_EVIDENCE_BYTES, type CaseEvidence, type CaseEvidenceContent } from '@/types/cases/cases';

const KNOWN_ERRORS = [
  'CASE_NOT_FOUND',
  'CASE_NOT_OPEN_FOR_EVIDENCE',
  'EVIDENCE_NOT_FOUND',
  'EVIDENCE_ENCODING_INVALID',
  'EVIDENCE_SIZE_INVALID',
  'EVIDENCE_TYPE_INVALID',
  'EVIDENCE_CONTENT_MISMATCH',
  'FORBIDDEN',
];

const NAMESPACE = 'hostCases';

const readAsBase64 = (file: File) => new Promise<string>((resolve, reject) => {
  const reader = new FileReader();
  reader.onload = () => {
    const result = String(reader.result ?? '');
    resolve(result.slice(result.indexOf(',') + 1));
  };
  reader.onerror = () => reject(reader.error);
  reader.readAsDataURL(file);
});

const saveContent = (fileName: string, mimeType: string, contentBase64: string) => {
  const bytes = Uint8Array.from(atob(contentBase64), (character) => character.charCodeAt(0));
  const url = URL.createObjectURL(new Blob([bytes], { type: mimeType }));
  const anchor = document.createElement('a');
  anchor.href = url;
  anchor.download = fileName;
  document.body.appendChild(anchor);
  anchor.click();
  anchor.remove();
  URL.revokeObjectURL(url);
};

export const useCaseEvidence = (caseId: string | null, canUpload: boolean, onChanged: () => void) => {
  const { t } = useTranslation();
  const translateRaw = t as unknown as (key: string, options?: Record<string, unknown>) => string;
  const translate = useCallback((key: string, options?: Record<string, unknown>) => translateRaw(key, { ...options, ns: NAMESPACE }), [translateRaw]);
  const { client } = useSession();
  const timezone = useCityTimezone();
  const fileInput = useRef<HTMLInputElement>(null);
  const [evidence, setEvidence] = useState<CaseEvidence[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');
  const [notice, setNotice] = useState('');
  const [uploading, setUploading] = useState(false);
  const [preview, setPreview] = useState<CaseEvidenceContent | null>(null);
  const [previewError, setPreviewError] = useState('');
  const [reloadToken, setReloadToken] = useState(0);

  const message = (code: string | null) => (code && KNOWN_ERRORS.includes(code) ? translate(`errors.${code}`) : translate('errors.generic'));

  useEffect(() => {
    setEvidence([]);
    setError('');
    setNotice('');
    setPreview(null);
    setPreviewError('');
  }, [caseId]);

  useEffect(() => {
    if (!caseId) return;
    let active = true;
    setLoading(true);
    setError('');
    void listEvidence(client, caseId).then((result) => {
      if (!active) return;
      setLoading(false);
      if (result.error || !result.data) {
        setError(message(result.error));
        return;
      }
      setEvidence(result.data);
    }).catch(() => {
      if (!active) return;
      setLoading(false);
      setError(translate('error'));
    });
    return () => {
      active = false;
    };
  }, [client, caseId, reloadToken, translate]);

  const upload = async (file: File) => {
    if (!caseId || !canUpload) return;
    setError('');
    setNotice('');
    if (file.size <= 0 || file.size > MAX_EVIDENCE_BYTES) {
      setError(translate('errors.EVIDENCE_SIZE_INVALID'));
      return;
    }
    if (!EVIDENCE_MIME_TYPES.includes(file.type)) {
      setError(translate('errors.EVIDENCE_TYPE_INVALID'));
      return;
    }
    setUploading(true);
    try {
      const contentBase64 = await readAsBase64(file);
      const result = await addEvidence(client, caseId, { fileName: file.name, mimeType: file.type, contentBase64 });
      if (result.error || !result.data) {
        setError(message(result.error));
        return;
      }
      setNotice(translate('evidenceUploaded'));
      setReloadToken((value) => value + 1);
      onChanged();
    } catch {
      setError(translate('errors.generic'));
    } finally {
      setUploading(false);
    }
  };

  const chooseFile = (event: ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    event.target.value = '';
    if (file) void upload(file);
  };

  const openPreview = async (item: CaseEvidence) => {
    setPreviewError('');
    const result = await getEvidence(client, item.id);
    if (result.error || !result.data) {
      setPreviewError(message(result.error));
      return;
    }
    setPreview(result.data);
  };

  const closePreview = () => {
    setPreview(null);
    setPreviewError('');
  };

  const downloadPreview = () => {
    if (!preview) return;
    saveContent(preview.evidence.fileName, preview.evidence.mimeType, preview.contentBase64);
  };

  const sizeText = (bytes: number) => {
    if (bytes >= 1024 * 1024) return translate('sizeMb', { size: (bytes / (1024 * 1024)).toFixed(1) });
    return translate('sizeKb', { size: Math.max(1, Math.ceil(bytes / 1024)) });
  };

  const previewKind = preview === null
    ? 'none'
    : preview.evidence.mimeType.startsWith('image/')
      ? 'image'
      : preview.evidence.mimeType === 'application/pdf'
        ? 'pdf'
        : 'other';

  return {
    evidence,
    loading,
    error,
    notice,
    uploading,
    canUpload,
    preview,
    previewKind,
    previewError,
    fileInput,
    pickFile: () => fileInput.current?.click(),
    chooseFile,
    openPreview,
    closePreview,
    downloadPreview,
    dateText: (value: string) => formatDateTime(value, timezone),
    addedBy: (userId: string) => translate('addedBy', { id: userId }),
    sizeText,
    labels: {
      evidence: translate('evidence'),
      noEvidence: translate('noEvidence'),
      upload: translate('evidenceUpload'),
      uploadHint: translate('evidenceHint'),
      preview: translate('preview'),
      previewTitle: translate('previewTitle'),
      previewUnavailable: translate('previewUnavailable'),
      download: translate('download'),
      closePreview: translate('close'),
    },
  };
};

export type CaseEvidenceView = ReturnType<typeof useCaseEvidence>;
