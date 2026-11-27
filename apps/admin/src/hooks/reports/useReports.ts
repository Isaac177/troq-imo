import { useEffect, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { formatCdf } from '@mungini/utils';
import { loadReport } from '@/services/reports/reports.api';
import type { ReportRange, ReportSummary } from '@/types/reports/reports';

const initialRange = (): ReportRange => {
  const query = new URLSearchParams(window.location.search);
  return { start: query.get('from'), end: query.get('to') };
};

export const useReports = () => {
  const { t, i18n } = useTranslation('adminReports');
  const { client, city } = useSession();
  const [range, setRange] = useState<ReportRange>(initialRange);
  const [summary, setSummary] = useState<ReportSummary | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(false);

  useEffect(() => {
    const url = new URL(window.location.href);
    for (const [key, value] of [['from', range.start], ['to', range.end]] as const) {
      if (value) url.searchParams.set(key, value); else url.searchParams.delete(key);
    }
    window.history.replaceState(window.history.state, '', url);
    let active = true;
    setLoading(true);
    setError(false);
    setSummary(null);
    void loadReport(client, range).then((data) => {
      if (active) setSummary(data);
    }).catch(() => {
      if (active) setError(true);
    }).finally(() => {
      if (active) setLoading(false);
    });
    return () => { active = false; };
  }, [client, city, range.start, range.end]);

  return {
    range, setRange, loading, error, summary,
    labels: {
      title: t('title'), subtitle: t('subtitle'), dates: t('dates'), error: t('error'),
      revenue: t('revenue'), reservations: t('reservations'), confirmed: t('confirmed'),
      checkedIn: t('checkedIn'), checkedOut: t('checkedOut'), cancelled: t('cancelled'),
      noShow: t('noShow'), reviews: t('reviews'), rating: t('rating'), unavailable: t('unavailable'),
    },
    revenue: formatCdf(summary?.grossRevenue, i18n.language, 0),
    rating: summary?.averageRating == null ? t('unavailable') : new Intl.NumberFormat(i18n.language, { maximumFractionDigits: 1 }).format(summary.averageRating),
  };
};
