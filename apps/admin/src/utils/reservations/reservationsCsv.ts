import { formatCdf, formatIsoDay } from '@mungini/utils';
import type { ReservationRow } from '@/types/reservations/reservations';

const BOM = '\uFEFF';

export type ReservationCsvHeaders = {
  reference: string;
  property: string;
  host: string;
  guest: string;
  checkIn: string;
  checkOut: string;
  state: string;
  total: string;
  commission: string;
};

export type ReservationCsvOptions = {
  headers: ReservationCsvHeaders;
  locale: string;
  propertyName: (row: ReservationRow) => string;
  stateLabel: (state: string) => string;
};

const quote = (value: string): string => `"${value.replace(/"/g, '""')}"`;

const line = (values: string[]): string => `${values.map(quote).join(',')}\r\n`;

const dayStamp = (): string => {
  const now = new Date();
  const month = String(now.getMonth() + 1).padStart(2, '0');
  const day = String(now.getDate()).padStart(2, '0');
  return `${now.getFullYear()}-${month}-${day}`;
};

export const reservationsCsvFilename = (): string => `mungini-admin-reservations-${dayStamp()}.csv`;

export const buildReservationsCsv = (rows: ReservationRow[], options: ReservationCsvOptions): string => {
  const { headers, locale, propertyName, stateLabel } = options;
  const header = line([
    headers.reference,
    headers.property,
    headers.host,
    headers.guest,
    headers.checkIn,
    headers.checkOut,
    headers.state,
    headers.total,
    headers.commission,
  ]);
  const body = rows.map((row) =>
    line([
      row.reference,
      propertyName(row),
      row.operatorName,
      row.guestName ?? '',
      formatIsoDay(row.checkIn),
      formatIsoDay(row.checkOut),
      stateLabel(row.state),
      formatCdf(row.totalAmount, locale),
      formatCdf(row.commissionAmount, locale),
    ]),
  );
  return `${BOM}${header}${body.join('')}`;
};

export const downloadCsv = (content: string, filename: string): void => {
  const blob = new Blob([content], { type: 'text/csv;charset=utf-8' });
  const url = URL.createObjectURL(blob);
  const anchor = document.createElement('a');
  anchor.href = url;
  anchor.download = filename;
  document.body.appendChild(anchor);
  anchor.click();
  anchor.remove();
  URL.revokeObjectURL(url);
};
