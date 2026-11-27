import { useEffect, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { formatDateTime } from '@mungini/utils';
import { listCities, listMessages, sendMessage } from '@/services/reservations/reservations.api';
import type { MessageView } from '@/types/reservations/messages';
import { markMessagesRead } from '@/services/messages/messages.api';

export const useReservationMessages = (reservationId: string | null, enabled: boolean, onRead?: () => void) => {
  const { t } = useTranslation('hostReservations');
  const { t: tc } = useTranslation('common');
  const translate = t as unknown as (key: string) => string;
  const { client, city } = useSession();
  const [messages, setMessages] = useState<MessageView[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [body, setBody] = useState('');
  const [sending, setSending] = useState(false);
  const [timezone, setTimezone] = useState('UTC');
  const [page, setPage] = useState(1);
  const [total, setTotal] = useState(0);

  useEffect(() => { setPage(1); }, [reservationId]);

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
    setMessages([]);
    setBody('');
    setError(null);
    if (!reservationId || !enabled) return;
    let active = true;
    setLoading(true);
    void listMessages(client, reservationId, page).then((result) => {
      if (!active) return;
      setLoading(false);
      if (result.error || !result.data) {
        setError(translate('messageError'));
        return;
      }
      setMessages(result.data.items);
      setTotal(result.data.total);
      const last = result.data.items[result.data.items.length - 1];
      if (last) void markMessagesRead(client, reservationId, last.id).then((marked) => {
        if (!active) return;
        if (marked) onRead?.();
        else setError(translate('messageError'));
      }).catch(() => { if (active) setError(translate('messageError')); });
    });
    return () => {
      active = false;
    };
  }, [client, reservationId, enabled, translate, page, onRead]);

  const submit = async () => {
    if (!reservationId || !body.trim() || sending) return;
    setSending(true);
    setError(null);
    const result = await sendMessage(client, reservationId, body.trim());
    setSending(false);
    if (result.error || !result.data) {
      setError(result.error === 'MESSAGE_BODY_INVALID' ? translate('messageBodyInvalid') : translate('messageError'));
      return;
    }
    setTotal((current) => current + 1);
    if (page * 50 >= total + 1) setMessages((current) => [...current, result.data as MessageView]);
    else setPage(Math.ceil((total + 1) / 50));
    setBody('');
  };

  return {
    messages,
    loading,
    error,
    body,
    setBody,
    sending,
    submit,
    page,
    canPrevious: page > 1,
    canNext: page * 50 < total,
    previous: () => setPage((current) => Math.max(1, current - 1)),
    next: () => setPage((current) => current + 1),
    time: (value: string) => formatDateTime(value, timezone),
    labels: {
      messages: translate('messages'),
      empty: translate('messagesEmpty'),
      placeholder: translate('messagePlaceholder'),
      send: translate('sendMessage'),
      error: translate('messageError'),
      previous: tc('previous'),
      next: tc('next'),
    },
  };
};
