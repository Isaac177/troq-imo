import { useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { exportReservations } from '@/services/reservations/reservations.api';
import type { HostReservation } from '@/types/reservations/reservations';
import { buildReservationsCsv, downloadCsv, todayStamp } from '@/utils/csv/reservationsCsv';

const EXPORT_PERMISSION = 'analytics.export';
const EXPORT_PAGE_SIZE = 100;

export type ReservationExportFilters = {
  hostId: string;
  propertyId: string;
  state: string;
  search: string;
};

export const useReservationsExport = (filters: ReservationExportFilters) => {
  const { t, i18n } = useTranslation('hostReservations');
  const translate = t as unknown as (key: string, options?: Record<string, unknown>) => string;
  const { client, can } = useSession();
  const [exporting, setExporting] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [notice, setNotice] = useState<string | null>(null);
  const locale = i18n.language.startsWith('fr') ? 'fr' : 'en';
  const target = {
    hostId: filters.hostId,
    propertyId: filters.propertyId === 'all' ? '' : filters.propertyId,
    state: filters.state === 'all' ? '' : filters.state,
    search: filters.search,
  };
  const canExport = target.hostId.length > 0 && can(EXPORT_PERMISSION, target.hostId);

  const failureMessage = (code: string | null) => (
    code === 'FORBIDDEN' ? translate('errors.FORBIDDEN') : translate('exportError')
  );

  const collect = async (): Promise<{ rows: HostReservation[]; error: string | null }> => {
    const rows: HostReservation[] = [];
    let total = 0;
    let page = 1;
    do {
      const result = await exportReservations(client, { ...target, locale, page, pageSize: EXPORT_PAGE_SIZE });
      if (result.error || !result.data) return { rows: [], error: result.error ?? 'EXPORT_FAILED' };
      rows.push(...result.data.items);
      total = result.data.total;
      page += 1;
      if (result.data.items.length === 0) break;
    } while (rows.length < total);
    return { rows, error: null };
  };

  const run = async () => {
    if (!canExport || exporting) return;
    setExporting(true);
    setError(null);
    setNotice(null);
    try {
      const { rows, error: code } = await collect();
      if (code) {
        setError(failureMessage(code));
        return;
      }
      if (rows.length === 0) {
        setError(translate('exportEmpty'));
        return;
      }
      const csv = buildReservationsCsv(rows, {
        locale,
        stateLabel: (state) => translate(`states.${state}`),
        headings: {
          reference: translate('csvHeadings.reference'),
          property: translate('csvHeadings.property'),
          unit: translate('csvHeadings.unit'),
          guest: translate('csvHeadings.guest'),
          checkIn: translate('csvHeadings.checkIn'),
          checkOut: translate('csvHeadings.checkOut'),
          nights: translate('csvHeadings.nights'),
          adults: translate('csvHeadings.adults'),
          children: translate('csvHeadings.children'),
          state: translate('csvHeadings.state'),
          total: translate('csvHeadings.total'),
          commission: translate('csvHeadings.commission'),
          net: translate('csvHeadings.net'),
        },
      });
      downloadCsv(`mungini-reservations-${todayStamp()}.csv`, csv);
      setNotice(translate('exportDone', { count: rows.length }));
    } catch {
      setError(translate('exportError'));
    } finally {
      setExporting(false);
    }
  };

  return {
    canExport,
    exporting,
    error,
    notice,
    run,
    labels: {
      export: translate('exportAction'),
      exporting: translate('exporting'),
    },
  };
};
