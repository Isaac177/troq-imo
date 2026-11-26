import { useEffect, useMemo, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { addDays, formatCdf } from '@mungini/utils';
import { hostApi, type DashboardFigures } from '@/services/host/host.api';

const today = () => new Date().toISOString().slice(0, 10);

export const useDashboard = () => {
  const { t, i18n } = useTranslation('host');
  const { session, client, city } = useSession();
  const api = useMemo(() => hostApi(client), [client]);
  const hosts = session?.hosts ?? [];
  const [hostId, setHostId] = useState(hosts[0]?.hostId ?? '');
  const [range, setRange] = useState<{ start: string | null; end: string | null }>({
    start: addDays(today(), -29),
    end: today(),
  });
  const [figures, setFigures] = useState<DashboardFigures | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!hostId && hosts[0]) setHostId(hosts[0].hostId);
    if (hostId && !hosts.some((host) => host.hostId === hostId)) setHostId(hosts[0]?.hostId ?? '');
  }, [hosts, hostId]);

  useEffect(() => {
    if (!hostId || !range.start || !range.end) return;
    let active = true;
    setLoading(true);
    setError(null);
    void api.dashboard(hostId, range.start, range.end).then((result) => {
      if (!active) return;
      setLoading(false);
      if (result.error) {
        setError(t('error'));
        setFigures(null);
        return;
      }
      setFigures(result.data);
    });
    return () => {
      active = false;
    };
  }, [api, hostId, range.start, range.end, city, t]);

  const occupancy = figures && figures.nightsAvailable > 0 ? Math.round((figures.nightsSold / figures.nightsAvailable) * 100) : 0;

  return {
    hosts,
    hostId,
    setHostId,
    range,
    setRange,
    figures,
    loading,
    error,
    city,
    labels: {
      title: t('dashboardTitle'),
      subtitle: t('dashboardSubtitle'),
      revenue: t('revenue'),
      bookings: t('bookings'),
      occupancy: t('occupancy'),
      pending: t('pendingRequests'),
      openCases: t('openCases'),
      noHost: t('noHost'),
      noHostAction: t('noHostAction'),
      error: t('error'),
    },
    values: {
      revenue: formatCdf(figures?.revenue, i18n.language, 0),
      bookings: String(figures?.bookings ?? 0),
      occupancy: `${occupancy}%`,
      pending: String(figures?.pendingRequests ?? 0),
      openCases: String(figures?.openCases ?? 0),
    },
  };
};
