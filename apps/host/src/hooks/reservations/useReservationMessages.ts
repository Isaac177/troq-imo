import { useEffect, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { formatDateTime } from '@mungini/utils';
import { listCities, listMessages, sendMessage } from '@/services/reservations/reservations.api';
import type { MessageView } from '@/types/reservations/messages';

export const useReservationMessages = (reservationId: string | null, enabled: boolean) => {
  const { t } = useTranslation('hostReservations');
  const translate = t as unknown as (key: string) => string;
  const { client, city } = useSession();
  const [messages, setMessages] = useState<MessageView[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [body, setBody] = useState('');
  const [sending, setSending] = useState(false);
  const [timezone, setTimezone] = useState('UTC');

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
    void listMessages(client, reservationId).then((result) => {
      if (!active) return;
      setLoading(false);
      if (result.error || !result.data) {
        setError(translate('messageError'));
        return;
      }
      setMessages(result.data.items);
    });
    return () => {
      active = false;
    };
  }, [client, reservationId, enabled, translate]);

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
    setMessages((current) => [...current, result.data as MessageView]);
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
    time: (value: string) => formatDateTime(value, timezone),
    labels: {
      messages: translate('messages'),
      empty: translate('messagesEmpty'),
      placeholder: translate('messagePlaceholder'),
      send: translate('sendMessage'),
      error: translate('messageError'),
    },
  };
};
