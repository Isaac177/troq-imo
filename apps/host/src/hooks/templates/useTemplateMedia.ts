import { useEffect, useState, type ChangeEvent } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { addTemplateMedia, listTemplateMedia, removeTemplateMedia, updateTemplateMedia } from '@/services/templates/templates.api';
import type { TemplateMedia } from '@/types/templates/templates';

const KNOWN_ERRORS = [
  'MEDIA_NOT_FOUND',
  'MEDIA_CLIENT_KEY_INVALID',
  'MEDIA_CONTENT_INVALID',
  'MEDIA_MIME_TYPE_INVALID',
  'MEDIA_ALT_TEXT_TOO_LONG',
  'MEDIA_COVER_CANNOT_BE_CLEARED',
  'MEDIA_COVER_NOT_READY',
  'MEDIA_VERSION_CONFLICT',
  'FORBIDDEN',
  'HOST_NOT_MEMBER',
];
const MAX_BYTES = 5 * 1024 * 1024;

type Picked = { fileName: string; mimeType: string; contentBase64: string };

export const useTemplateMedia = (templateId: string | null, onChanged: () => void) => {
  const { t } = useTranslation('hostTemplates');
  const translate = t as unknown as (key: string) => string;
  const { client } = useSession();
  const [media, setMedia] = useState<TemplateMedia[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [notice, setNotice] = useState<string | null>(null);
  const [uploadOpen, setUploadOpen] = useState(false);
  const [picked, setPicked] = useState<Picked | null>(null);
  const [altEn, setAltEn] = useState('');
  const [altFr, setAltFr] = useState('');
  const [cover, setCover] = useState(false);
  const [busy, setBusy] = useState(false);
  const [formError, setFormError] = useState<string | null>(null);
  const [removeTarget, setRemoveTarget] = useState<TemplateMedia | null>(null);

  const message = (code: string | null) => (code && KNOWN_ERRORS.includes(code) ? translate(`errors.${code}`) : translate('errors.generic'));

  const load = async () => {
    if (!templateId) return;
    const result = await listTemplateMedia(client, templateId);
    if (result.error) {
      setError(message(result.error));
      return;
    }
    setMedia([...(result.data ?? [])].sort((left, right) => left.sortOrder - right.sortOrder));
  };

  useEffect(() => {
    setMedia([]);
    setNotice(null);
    setError(null);
    setUploadOpen(false);
    setRemoveTarget(null);
    if (!templateId) return;
    let active = true;
    setLoading(true);
    void load().finally(() => {
      if (active) setLoading(false);
    });
    return () => {
      active = false;
    };
  }, [client, templateId]);

  const openUpload = () => {
    setPicked(null);
    setAltEn('');
    setAltFr('');
    setCover(media.length === 0);
    setFormError(null);
    setUploadOpen(true);
  };

  const closeUpload = () => {
    setUploadOpen(false);
    setFormError(null);
  };

  const pickFile = (file: File) => {
    setFormError(null);
    if (file.size > MAX_BYTES) {
      setFormError(message('MEDIA_CONTENT_INVALID'));
      return;
    }
    if (!['image/jpeg', 'image/png', 'image/webp'].includes(file.type)) {
      setFormError(message('MEDIA_MIME_TYPE_INVALID'));
      return;
    }
    const reader = new FileReader();
    reader.onload = () => {
      const value = typeof reader.result === 'string' ? reader.result : '';
      const contentBase64 = value.includes(',') ? value.slice(value.indexOf(',') + 1) : value;
      setPicked({ fileName: file.name, mimeType: file.type, contentBase64 });
    };
    reader.onerror = () => setFormError(translate('errors.generic'));
    reader.readAsDataURL(file);
  };

  const chooseFile = (event: ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (file) pickFile(file);
  };

  const submit = async () => {
    if (!templateId || !picked) return;
    setBusy(true);
    setFormError(null);
    const result = await addTemplateMedia(client, templateId, { ...picked, altEn, altFr });
    if (result.error || !result.data) {
      setBusy(false);
      setFormError(message(result.error));
      return;
    }
    if (cover) {
      await updateTemplateMedia(client, templateId, result.data.id, { expectedVersion: result.data.version, isCover: true });
    }
    setBusy(false);
    setUploadOpen(false);
    setNotice(translate('media.uploaded'));
    await load();
    onChanged();
  };

  const setCoverPhoto = async (photo: TemplateMedia) => {
    if (!templateId) return;
    setError(null);
    const result = await updateTemplateMedia(client, templateId, photo.id, { expectedVersion: photo.version, isCover: true });
    if (result.error) {
      setError(message(result.error));
      return;
    }
    setNotice(translate('media.coverSet'));
    await load();
    onChanged();
  };

  const move = async (photo: TemplateMedia, direction: -1 | 1) => {
    if (!templateId) return;
    const index = media.findIndex((item) => item.id === photo.id);
    const neighbor = media[index + direction];
    if (!neighbor) return;
    setError(null);
    const first = await updateTemplateMedia(client, templateId, photo.id, { expectedVersion: photo.version, sortOrder: neighbor.sortOrder });
    if (first.error) {
      setError(message(first.error));
      return;
    }
    const second = await updateTemplateMedia(client, templateId, neighbor.id, { expectedVersion: neighbor.version, sortOrder: photo.sortOrder });
    if (second.error) setError(message(second.error));
    await load();
    onChanged();
  };

  const confirmRemove = async () => {
    if (!templateId || !removeTarget) return;
    setBusy(true);
    const result = await removeTemplateMedia(client, templateId, removeTarget.id, removeTarget.version);
    setBusy(false);
    if (result.error) {
      setError(message(result.error));
      setRemoveTarget(null);
      return;
    }
    setRemoveTarget(null);
    setNotice(translate('media.removed'));
    await load();
    onChanged();
  };

  return {
    media,
    loading,
    error,
    notice,
    uploadOpen,
    openUpload,
    closeUpload,
    picked,
    chooseFile,
    altEn,
    setAltEn,
    altFr,
    setAltFr,
    cover,
    setCover,
    busy,
    formError,
    submit,
    setCoverPhoto,
    move,
    removeTarget,
    setRemoveTarget,
    confirmRemove,
    cancelRemove: () => setRemoveTarget(null),
    altFor: (photo: TemplateMedia) => photo.altEn || photo.altFr || photo.fileName,
    statusLabel: (status: string) => {
      const key = `media.statuses.${status}`;
      const label = translate(key);
      return label === key ? status : label;
    },
    labels: {
      title: translate('photos'),
      addPhoto: translate('media.addPhoto'),
      choosePhoto: translate('media.choosePhoto'),
      altEn: translate('media.altEn'),
      altFr: translate('media.altFr'),
      coverPhoto: translate('media.coverPhoto'),
      setCover: translate('media.setCover'),
      cover: translate('media.cover'),
      moveUp: translate('media.moveUp'),
      moveDown: translate('media.moveDown'),
      removePhoto: translate('media.removePhoto'),
      removeBody: translate('media.removeBody'),
      noPhotos: translate('media.noPhotos'),
      upload: translate('media.upload'),
      uploaded: translate('media.uploaded'),
      cancel: translate('cancel'),
      dismiss: translate('media.dismiss'),
    },
  };
};

export type TemplateMediaView = ReturnType<typeof useTemplateMedia>;
