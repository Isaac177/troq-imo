import { useEffect, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { Button, type TableColumn } from '@mungini/ui-web';
import { formatDateTime } from '@mungini/utils';
import { hostApi } from '@/services/host/host.api';
import { listCities, listReviews, replyToReview } from '@/services/reviews/reviews.api';
import type { PropertyOption } from '@/types/rates/rates';
import type { Review } from '@/types/reviews/reviews';

const PAGE_SIZE = 20;
const KNOWN_ERRORS = ['REVIEW_NOT_APPROVED', 'REVIEW_NOT_FOUND', 'REVIEW_TEXT_REQUIRED', 'REVIEW_TEXT_TOO_LONG', 'FORBIDDEN'];

export const useHostReviews = () => {
  const { t, i18n } = useTranslation('hostReviews');
  const { t: th } = useTranslation('host');
  const translate = t as unknown as (key: string) => string;
  const translateHost = th as unknown as (key: string) => string;
  const { session, client, city } = useSession();
  const hosts = session?.hosts ?? [];
  const [hostId, setHostId] = useState(hosts[0]?.hostId ?? '');
  const [properties, setProperties] = useState<PropertyOption[]>([]);
  const [propertyId, setPropertyId] = useState('');
  const [status, setStatus] = useState('all');
  const [rows, setRows] = useState<Review[]>([]);
  const [total, setTotal] = useState(0);
  const [page, setPage] = useState(1);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [notice, setNotice] = useState<string | null>(null);
  const [replyTarget, setReplyTarget] = useState<Review | null>(null);
  const [draft, setDraft] = useState('');
  const [busy, setBusy] = useState(false);
  const [formError, setFormError] = useState<string | null>(null);
  const [timezone, setTimezone] = useState('UTC');
  const french = i18n.language.startsWith('fr');

  const message = (code: string | null) => (code && KNOWN_ERRORS.includes(code) ? translate(`errors.${code}`) : translate('errors.generic'));

  useEffect(() => {
    if (!hosts.some((host) => host.hostId === hostId)) setHostId(hosts[0]?.hostId ?? '');
  }, [hosts, hostId]);

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
    if (!hostId) {
      setProperties([]);
      setPropertyId('');
      return;
    }
    let active = true;
    void hostApi(client).properties(hostId, 1, 100, '').then((result) => {
      if (!active || result.error) return;
      const options = (result.data?.items ?? []).map((property) => ({ value: property.id, label: (french ? property.nameFr : property.nameEn) || property.nameEn || property.nameFr }));
      setProperties(options);
      setPropertyId((current) => (options.some((option) => option.value === current) ? current : options[0]?.value ?? ''));
    });
    return () => {
      active = false;
    };
  }, [client, hostId, french]);

  useEffect(() => {
    setPage(1);
  }, [propertyId, status]);

  useEffect(() => {
    if (!propertyId) return;
    let active = true;
    setLoading(true);
    setError(null);
    void listReviews(client, propertyId, status === 'all' ? '' : status, page, PAGE_SIZE).then((result) => {
      if (!active) return;
      setLoading(false);
      if (result.error || !result.data) {
        setError(message(result.error));
        setRows([]);
        setTotal(0);
        return;
      }
      setRows(result.data.items);
      setTotal(result.data.total);
    });
    return () => {
      active = false;
    };
  }, [client, propertyId, status, page, translate]);

  const unitName = (review: Review) => {
    const name = french ? review.unitTypeNameFr : review.unitTypeNameEn;
    return name || review.unitTypeNameEn || review.unitTypeNameFr || '';
  };
  const openReply = (review: Review) => {
    setReplyTarget(review);
    setDraft(review.operatorReply ?? '');
    setFormError(null);
  };
  const submit = async () => {
    if (!replyTarget) return;
    setBusy(true);
    setFormError(null);
    const result = await replyToReview(client, replyTarget.id, draft);
    setBusy(false);
    if (result.error || !result.data) {
      setFormError(message(result.error));
      return;
    }
    setRows((current) => current.map((review) => (review.id === result.data?.id ? (result.data as Review) : review)));
    setReplyTarget(null);
    setNotice(translate('saved'));
  };

  const columns: TableColumn<Review>[] = [
    { key: 'rating', header: translate('rating'), cell: (row) => <span className="tabular-nums font-medium">{row.rating} / 5</span> },
    {
      key: 'review',
      header: translate('comment'),
      cell: (row) => <div className="min-w-0"><p className="truncate font-medium text-foreground">{unitName(row) || translate('unit')}</p><p className="truncate text-label text-muted-foreground">{row.comment || ''}</p></div>,
    },
    { key: 'date', header: translate('date'), hideBelow: 'sm', cell: (row) => <span className="whitespace-nowrap tabular-nums">{formatDateTime(row.createdAt, timezone)}</span> },
    { key: 'status', header: translate('status'), hideBelow: 'sm', cell: (row) => translate(`statuses.${row.status}`) },
    { key: 'replyState', header: translate('replyStatus'), hideBelow: 'sm', cell: (row) => (row.operatorReply ? translate('replied') : translate('notReplied')) },
    {
      key: 'actions',
      header: '',
      align: 'right',
      cell: (row) => (row.status === 'APPROVED' ? <Button size="sm" variant="outline" onClick={() => openReply(row)}>{translate('reply')}</Button> : null),
    },
  ];

  return {
    hosts, hasHost: hosts.length > 0, hostId, setHostId,
    properties, propertyId, setPropertyId,
    status, setStatus, rows, total, page, setPage, pageSize: PAGE_SIZE, loading, error, notice, columns,
    replyTarget, setReplyTarget, draft, setDraft, busy, formError, submit, openReply,
    statusOptions: [
      { value: 'all', label: translate('allStatuses') },
      { value: 'APPROVED', label: translate('statuses.APPROVED') },
      { value: 'PENDING', label: translate('statuses.PENDING') },
      { value: 'HIDDEN', label: translate('statuses.HIDDEN') },
    ],
    replyText: replyTarget?.comment ?? '',
    labels: {
      title: translate('title'), subtitle: translate('subtitle'), property: translate('property'), chooseProperty: translate('chooseProperty'),
      status: translate('status'), rating: translate('rating'), comment: translate('comment'), date: translate('date'), reply: translate('reply'),
      empty: translate('empty'), error: translate('error'), loading: translate('loading'), replyTitle: translate('replyTitle'),
      replyPlaceholder: translate('replyPlaceholder'), sendReply: translate('sendReply'), saved: translate('saved'), close: translate('close'),
      noHost: translateHost('noHost'), noHostAction: translateHost('noHostAction'),
    },
  };
};

export type HostReviewsView = ReturnType<typeof useHostReviews>;
