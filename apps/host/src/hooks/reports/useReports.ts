import { useEffect, useState } from 'react';
import { useSearchParams } from 'react-router-dom';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { formatCdf } from '@mungini/utils';
import { hostApi } from '@/services/host/host.api';
import { loadReport } from '@/services/reports/reports.api';
import type { ReportProperty, ReportRange, ReportSummary } from '@/types/reports/reports';

export const useReports = () => {
  const { t, i18n } = useTranslation('hostReports');
  const { client, city, session, can } = useSession();
  const [params, setParams] = useSearchParams();
  const hosts = (session?.hosts ?? []).filter((host) => can('analytics.read', host.hostId));
  const selectedHost = params.get('host') ?? '';
  const hostId = hosts.some((host) => host.hostId === selectedHost) ? selectedHost : hosts[0]?.hostId ?? '';
  const [properties, setProperties] = useState<ReportProperty[]>([]);
  const selectedProperty = params.get('property') ?? 'all';
  const propertyId = properties.some((property) => property.id === selectedProperty) ? selectedProperty : '';
  const range: ReportRange = { start: params.get('from'), end: params.get('to') };
  const [summary, setSummary] = useState<ReportSummary | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(false);

  const update = (values: Record<string, string | null>) => {
    const next = new URLSearchParams(params);
    for (const [key, value] of Object.entries(values)) {
      if (value) next.set(key, value); else next.delete(key);
    }
    setParams(next, { replace: true });
  };

  useEffect(() => {
    if (!hostId) {
      setProperties([]);
      return;
    }
    let active = true;
    setProperties([]);
    void hostApi(client).properties(hostId, 1, 100, '').then(({ data }) => {
      if (active && data) setProperties(data.items.map((property) => ({ id: property.id, nameEn: property.nameEn, nameFr: property.nameFr })));
    }).catch(() => {
      if (active) setProperties([]);
    });
    return () => { active = false; };
  }, [client, city, hostId]);

  useEffect(() => {
    if (!hostId) { setSummary(null); return; }
    let active = true;
    setLoading(true);
    setError(false);
    setSummary(null);
    void loadReport(client, hostId, { start: range.start, end: range.end }, propertyId || undefined).then((data) => {
      if (active) setSummary(data);
    }).catch(() => {
      if (active) setError(true);
    }).finally(() => {
      if (active) setLoading(false);
    });
    return () => { active = false; };
  }, [client, city, hostId, propertyId, range.start, range.end]);

  const propertyName = (property: ReportProperty) => i18n.language.startsWith('fr')
    ? property.nameFr || property.nameEn
    : property.nameEn || property.nameFr;

  return {
    hosts: hosts.map((host) => ({ value: host.hostId, label: host.displayName })),
    hostId, setHostId: (id: string) => update({ host: id, property: null }),
    propertyOptions: [{ value: 'all', label: t('allProperties') }, ...properties.map((property) => ({ value: property.id, label: propertyName(property) }))],
    propertyId: propertyId || 'all', setPropertyId: (id: string) => update({ property: id === 'all' ? null : id }),
    range, setRange: (value: ReportRange) => update({ from: value.start, to: value.end }),
    summary, loading, error,
    revenue: formatCdf(summary?.grossRevenue, i18n.language, 0),
    rating: summary?.averageRating == null ? t('unavailable') : new Intl.NumberFormat(i18n.language, { maximumFractionDigits: 1 }).format(summary.averageRating),
    labels: {
      title: t('title'), subtitle: t('subtitle'), host: t('host'), property: t('property'), dates: t('dates'), error: t('error'), empty: t('empty'),
      revenue: t('revenue'), reservations: t('reservations'), confirmed: t('confirmed'), checkedIn: t('checkedIn'),
      checkedOut: t('checkedOut'), cancelled: t('cancelled'), noShow: t('noShow'), reviews: t('reviews'), rating: t('rating'),
    },
  };
};
