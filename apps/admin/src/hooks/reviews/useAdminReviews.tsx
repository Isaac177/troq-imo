import { useEffect, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { Button, useDebouncedValue, type TableColumn } from '@mungini/ui-web';
import { formatDateTime } from '@mungini/utils';
import { decideReview, listCities, listReviews } from '@/services/reviews/reviews.api';
import type { AdminReview } from '@/types/reviews/reviews';

const PAGE_SIZE = 20;
const KNOWN_ERRORS = ['REVIEW_NOT_FOUND', 'REVIEW_MODERATION_STATUS_INVALID', 'REVIEW_HIDE_REASON_REQUIRED', 'FORBIDDEN'];

export const useAdminReviews = () => {
  const { t, i18n } = useTranslation('adminReviews');
  const translate = t as unknown as (key: string) => string;
  const { client, city } = useSession();
  const [search, setSearch] = useState('');
  const term = useDebouncedValue(search, 250);
  const [status, setStatus] = useState('all');
  const [rating, setRating] = useState('all');
  const [page, setPage] = useState(1);
  const [rows, setRows] = useState<AdminReview[]>([]);
  const [total, setTotal] = useState(0);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [notice, setNotice] = useState<string | null>(null);
  const [target, setTarget] = useState<AdminReview | null>(null);
  const [decision, setDecision] = useState('APPROVED');
  const [reason, setReason] = useState('');
  const [busy, setBusy] = useState(false);
  const [formError, setFormError] = useState<string | null>(null);
  const [timezone, setTimezone] = useState('UTC');
  const french = i18n.language.startsWith('fr');

  const message = (code: string | null) => (code && KNOWN_ERRORS.includes(code) ? translate(`errors.${code}`) : translate('errors.generic'));

  useEffect(() => {
    setPage(1);
  }, [term, status, rating, city]);

  useEffect(() => {
    let active = true;
    setLoading(true);
    setError(null);
    void listReviews(client, { status: status === 'all' ? '' : status, rating: rating === 'all' ? '' : rating, search: term, page, pageSize: PAGE_SIZE }).then((result) => {
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
  }, [client, city, status, rating, term, page, translate]);

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

  const open = (row: AdminReview) => {
    setTarget(row);
    setDecision('APPROVED');
    setReason('');
    setFormError(null);
  };
  const submit = async () => {
    if (!target) return;
    if (decision === 'HIDDEN' && !reason) {
      setFormError(translate('errors.REVIEW_HIDE_REASON_REQUIRED'));
      return;
    }
    setBusy(true);
    setFormError(null);
    const result = await decideReview(client, target.id, decision, decision === 'HIDDEN' ? reason : null);
    setBusy(false);
    if (result.error || !result.data) {
      setFormError(message(result.error));
      return;
    }
    setRows((current) => current.map((row) => (row.id === target.id ? { ...row, status: result.data?.status ?? row.status } : row)));
    setTarget(null);
    setNotice(translate('saved'));
  };

  const propertyName = (row: AdminReview) => (french ? row.propertyNameFr : row.propertyNameEn) || row.propertyNameEn || row.propertyNameFr;
  const columns: TableColumn<AdminReview>[] = [
    {
      key: 'review',
      header: translate('property'),
      cell: (row) => (
        <div className="min-w-0">
          <p className="truncate font-medium text-foreground">{propertyName(row)}</p>
          <p className="truncate text-label text-muted-foreground">{row.guestName || translate('guest')} · {row.reservationReference}</p>
          <p className="truncate text-label text-muted-foreground sm:hidden">{row.comment || ''}</p>
        </div>
      ),
    },
    { key: 'rating', header: translate('rating'), cell: (row) => <span className="tabular-nums font-medium">{row.rating} / 5</span> },
    { key: 'status', header: translate('status'), hideBelow: 'sm', cell: (row) => translate(`statuses.${row.status}`) },
    { key: 'date', header: translate('date'), hideBelow: 'sm', cell: (row) => <span className="whitespace-nowrap tabular-nums">{formatDateTime(row.createdAt, timezone)}</span> },
    { key: 'actions', header: '', align: 'right', cell: (row) => <Button size="sm" variant="outline" onClick={() => open(row)}>{translate('moderate')}</Button> },
  ];

  return {
    search, setSearch, status, setStatus, rating, setRating,
    page, setPage, pageSize: PAGE_SIZE, rows, total, loading, error, notice, columns,
    target, setTarget, decision, setDecision, reason, setReason, busy, formError, submit, open,
    selectedComment: target?.comment ?? '',
    selectedReply: target?.operatorReply ?? '',
    statusOptions: [
      { value: 'all', label: translate('allStatuses') },
      { value: 'PENDING', label: translate('statuses.PENDING') },
      { value: 'APPROVED', label: translate('statuses.APPROVED') },
      { value: 'HIDDEN', label: translate('statuses.HIDDEN') },
    ],
    ratingOptions: [
      { value: 'all', label: translate('allRatings') },
      ...[5, 4, 3, 2, 1].map((value) => ({ value: String(value), label: `${value} / 5` })),
    ],
    decisionOptions: [
      { value: 'APPROVED', label: translate('approve') },
      { value: 'HIDDEN', label: translate('hide') },
    ],
    reasonOptions: ['ABUSIVE', 'SPAM', 'OFF_TOPIC', 'PRIVACY', 'OTHER'].map((value) => ({ value, label: translate(`reasons.${value}`) })),
    labels: {
      title: translate('title'), subtitle: translate('subtitle'), search: translate('search'), status: translate('status'),
      rating: translate('rating'), property: translate('property'), guest: translate('guest'), reference: translate('reference'),
      date: translate('date'), comment: translate('comment'), reply: translate('reply'), moderate: translate('moderate'),
      moderationTitle: translate('moderationTitle'), moderationBody: translate('moderationBody'), decision: translate('decision'),
      approve: translate('approve'), hide: translate('hide'), reason: translate('reason'), chooseReason: translate('chooseReason'),
      save: translate('save'), close: translate('close'), saved: translate('saved'), error: translate('error'),
      loading: translate('loading'), empty: translate('empty'),
    },
  };
};

export type AdminReviewsView = ReturnType<typeof useAdminReviews>;
