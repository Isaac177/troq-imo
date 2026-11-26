import { useEffect, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { useDebouncedValue, type TableColumn } from '@mungini/ui-web';
import { formatCdf, formatIsoDay } from '@mungini/utils';
import { listReservations, listReservationsExport } from '@/services/reservations/reservations.api';
import { buildReservationsCsv, downloadCsv, reservationsCsvFilename } from '@/utils/reservations/reservationsCsv';
import type { ReservationRow } from '@/types/reservations/reservations';

const pageSize = 20;
const exportPageSize = 100;

export const useReservations = () => {
  const { t, i18n } = useTranslation('admin');
  const { t: tr } = useTranslation('adminReservations');
  const { client, city } = useSession();
  const [search, setSearch] = useState('');
  const [state, setState] = useState('all');
  const [selected, setSelected] = useState<string | null>(null);
  const term = useDebouncedValue(search, 250);
  const [page, setPage] = useState(1);
  const [rows, setRows] = useState<ReservationRow[]>([]);
  const [total, setTotal] = useState(0);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(false);
  const [exporting, setExporting] = useState(false);
  const [exportError, setExportError] = useState<string | null>(null);
  const [notice, setNotice] = useState<string | null>(null);

  useEffect(() => { setPage(1); }, [term, city, state]);
  useEffect(() => {
    let active = true;
    setLoading(true);
    setError(false);
    void listReservations(client, term, state === 'all' ? '' : state, page, pageSize).then(({ data, error: failure }) => {
      if (!active) return;
      setLoading(false);
      if (failure || !data) { setError(true); setRows([]); setTotal(0); return; }
      setRows(data.data.items);
      setTotal(data.data.total);
    }).catch(() => { if (active) { setLoading(false); setError(true); setRows([]); setTotal(0); } });
    return () => { active = false; };
  }, [client, city, term, state, page]);

  const labels = {
    title: t('reservationsTitle'), subtitle: t('reservationsSubtitle'), search: t('search'),
    property: t('reservationProperty'), guest: t('guest'), dates: t('reservationDates'),
    amount: t('reservationAmount'), status: t('status'), empty: t('reservationsEmpty'), error: t('error'),
    exportAction: tr('exportAction'),
  };
  const stateOptions = [
    { value: 'all', label: tr('all') }, { value: 'PENDING_PAYMENT', label: tr('pendingPayment') },
    { value: 'REQUESTED', label: tr('requested') }, { value: 'CONFIRMED', label: tr('confirmed') },
    { value: 'DECLINED', label: tr('declined') }, { value: 'CANCELLED', label: tr('cancelled') },
    { value: 'EXPIRED', label: tr('expired') }, { value: 'CHECKED_IN', label: tr('checkedIn') },
    { value: 'CHECKED_OUT', label: tr('checkedOut') }, { value: 'NO_SHOW', label: tr('noShow') },
  ];
  const stateLabel = (value: string) => stateOptions.find(option => option.value === value)?.label ?? tr('other');
  const propertyName = (row: ReservationRow) => (i18n.language.startsWith('fr') ? row.propertyNameFr || row.propertyNameEn : row.propertyNameEn || row.propertyNameFr);
  const columns: TableColumn<ReservationRow>[] = [
    { key: 'property', header: labels.property, cell: row => <div className="min-w-0"><p className="truncate font-medium">{propertyName(row)}</p><p className="truncate text-label text-muted-foreground">{row.reference}</p></div> },
    { key: 'status', header: tr('status'), cell: row => stateLabel(row.state), hideBelow: 'sm' },
    { key: 'guest', header: labels.guest, cell: row => row.guestName || '', hideBelow: 'sm' },
    { key: 'dates', header: labels.dates, cell: row => <span className="tabular-nums">{formatIsoDay(row.checkIn)} – {formatIsoDay(row.checkOut)}</span>, hideBelow: 'sm' },
    { key: 'amount', header: labels.amount, align: 'right', cell: row => <span className="tabular-nums">{formatCdf(row.totalAmount, i18n.language)}</span> },
  ];

  const exportReservations = async () => {
    setExporting(true);
    setExportError(null);
    setNotice(null);
    const collected: ReservationRow[] = [];
    try {
      let requestedPage = 1;
      let matching = Number.POSITIVE_INFINITY;
      while (collected.length < matching) {
        const result = await listReservationsExport(client, term, state === 'all' ? '' : state, requestedPage, exportPageSize);
        if (result.error || !result.data) {
          setExportError(tr('exportError'));
          return;
        }
        matching = result.data.total;
        if (result.data.items.length === 0) break;
        collected.push(...result.data.items);
        requestedPage += 1;
      }
      const csv = buildReservationsCsv(collected, {
        headers: {
          reference: tr('reference'), property: tr('property'), host: tr('host'), guest: tr('guest'),
          checkIn: tr('checkIn'), checkOut: tr('checkOut'), state: tr('status'), total: tr('amount'), commission: tr('commission'),
        },
        locale: i18n.language,
        propertyName,
        stateLabel,
      });
      downloadCsv(csv, reservationsCsvFilename());
      setNotice(tr('exported').replace('{count}', String(collected.length)));
    } catch {
      setExportError(tr('exportError'));
    } finally {
      setExporting(false);
    }
  };

  return {
    labels, columns, search, setSearch, state, setState, stateOptions, statusLabel: tr('status'),
    selected, setSelected, page, setPage, pageSize, rows, total, loading, error,
    exporting, exportError, notice, exportReservations,
  };
};
