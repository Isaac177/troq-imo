import { useEffect, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { addMedia, listMedia, removeMedia, updateMedia } from '@/services/properties/properties.api';
import type { PropertyMedia } from '@/services/properties/properties.api';

const KNOWN_ERRORS = ['MEDIA_NOT_FOUND', 'MEDIA_CLIENT_KEY_INVALID', 'MEDIA_CONTENT_INVALID', 'MEDIA_MIME_TYPE_INVALID', 'MEDIA_ALT_TEXT_TOO_LONG', 'MEDIA_COVER_CANNOT_BE_CLEARED', 'MEDIA_COVER_NOT_READY', 'MEDIA_VERSION_CONFLICT', 'FORBIDDEN'];
const MAX_BYTES = 5 * 1024 * 1024;

type Picked = { fileName: string; mimeType: string; contentBase64: string };

export const usePropertyMedia = (propertyId: string | null, onChanged: () => void) => {
  const { t } = useTranslation('host');
  const translate = t as unknown as (key: string) => string;
  const { client } = useSession();
  const [media, setMedia] = useState<PropertyMedia[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [notice, setNotice] = useState<string | null>(null);
  const [open, setOpen] = useState(false);
  const [picked, setPicked] = useState<Picked | null>(null);
  const [altEn, setAltEn] = useState('');
  const [altFr, setAltFr] = useState('');
  const [cover, setCover] = useState(false);
  const [busy, setBusy] = useState(false);
  const [formError, setFormError] = useState<string | null>(null);
  const [removeTarget, setRemoveTarget] = useState<PropertyMedia | null>(null);

  const message = (code: string | null) => (code && KNOWN_ERRORS.includes(code) ? translate(`photoErrors.${code}`) : translate('photoErrors.generic'));

  const load = async () => {
    if (!propertyId) return;
    const result = await listMedia(client, propertyId);
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
    if (!propertyId) return;
    let active = true;
    setLoading(true);
    void load().finally(() => {
      if (active) setLoading(false);
    });
    return () => {
      active = false;
    };
  }, [client, propertyId]);

  const openUpload = () => {
    setPicked(null);
    setAltEn('');
    setAltFr('');
    setCover(media.length === 0);
    setFormError(null);
    setOpen(true);
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
    reader.onerror = () => setFormError(translate('photoErrors.generic'));
    reader.readAsDataURL(file);
  };

  const submit = async () => {
    if (!propertyId || !picked) return;
    setBusy(true);
    setFormError(null);
    const result = await addMedia(client, propertyId, { ...picked, altEn, altFr });
    if (result.error || !result.data) {
      setBusy(false);
      setFormError(message(result.error));
      return;
    }
    if (cover) {
      await updateMedia(client, propertyId, result.data.id, { expectedVersion: result.data.version, isCover: true });
    }
    setBusy(false);
    setOpen(false);
    setNotice(translate('photoUploaded'));
    await load();
    onChanged();
  };

  const setCoverPhoto = async (photo: PropertyMedia) => {
    if (!propertyId) return;
    setError(null);
    const result = await updateMedia(client, propertyId, photo.id, { expectedVersion: photo.version, isCover: true });
    if (result.error) {
      setError(message(result.error));
      return;
    }
    setNotice(translate('coverSet'));
    await load();
    onChanged();
  };

  const move = async (photo: PropertyMedia, direction: -1 | 1) => {
    if (!propertyId) return;
    const index = media.findIndex((item) => item.id === photo.id);
    const neighbor = media[index + direction];
    if (!neighbor) return;
    setError(null);
    const first = await updateMedia(client, propertyId, photo.id, { expectedVersion: photo.version, sortOrder: neighbor.sortOrder });
    if (first.error) {
      setError(message(first.error));
      return;
    }
    const second = await updateMedia(client, propertyId, neighbor.id, { expectedVersion: neighbor.version, sortOrder: photo.sortOrder });
    if (second.error) {
      setError(message(second.error));
    }
    await load();
    onChanged();
  };

  const confirmRemove = async () => {
    if (!propertyId || !removeTarget) return;
    setBusy(true);
    const result = await removeMedia(client, propertyId, removeTarget.id, removeTarget.version);
    setBusy(false);
    if (result.error) {
      setError(message(result.error));
      setRemoveTarget(null);
      return;
    }
    setRemoveTarget(null);
    setNotice(translate('photoRemoved'));
    await load();
    onChanged();
  };

  return {
    media, loading, error, notice, open, setOpen, openUpload, picked, pickFile, altEn, setAltEn, altFr, setAltFr,
    cover, setCover, busy, formError, submit, setCoverPhoto, move, removeTarget, setRemoveTarget, confirmRemove,
    altFor: (photo: PropertyMedia) => photo.altEn || photo.altFr || photo.fileName,
    statusLabel: (status: string) => {
      const key = `photoStatuses.${status}`;
      const label = translate(key);
      return label === key ? status : label;
    },
    labels: {
      propertyMedia: translate('propertyMedia'), addPhoto: translate('addPhoto'), choosePhoto: translate('choosePhoto'),
      photoAltEn: translate('photoAltEn'), photoAltFr: translate('photoAltFr'), cover: translate('coverPhoto'), setCover: translate('setCover'),
      moveUp: translate('moveUp'), moveDown: translate('moveDown'), removePhoto: translate('removePhoto'), noPhotos: translate('noPhotos'),
      confirmRemoveTitle: translate('confirmRemovePhotoTitle'), confirmRemoveBody: translate('confirmRemovePhotoBody'),
      close: translate('close'), dismiss: translate('close'), error: translate('photoErrors.generic'),
    },
  };
};

export type PropertyMediaView = ReturnType<typeof usePropertyMedia>;
