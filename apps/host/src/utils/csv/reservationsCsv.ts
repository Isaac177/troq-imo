import { formatCdf, formatIsoDay } from '@mungini/utils';
import type { HostReservation } from '@/types/reservations/reservations';

type CsvValue = string | number | null | undefined;

export type ReservationsCsvHeadings = {
  reference: string;
  property: string;
  unit: string;
  guest: string;
  checkIn: string;
  checkOut: string;
  nights: string;
  adults: string;
  children: string;
  state: string;
  total: string;
  commission: string;
  net: string;
};

export type ReservationsCsvContext = {
  locale: string;
  stateLabel: (state: string) => string;
  headings: ReservationsCsvHeadings;
};

const quote = (value: CsvValue): string => `"${String(value ?? '').replace(/"/g, '""')}"`;

const line = (values: CsvValue[]): string => values.map(quote).join(',');

export const buildReservationsCsv = (rows: HostReservation[], context: ReservationsCsvContext): string => {
  const header = line([
    context.headings.reference,
    context.headings.property,
    context.headings.unit,
    context.headings.guest,
    context.headings.checkIn,
    context.headings.checkOut,
    context.headings.nights,
    context.headings.adults,
    context.headings.children,
    context.headings.state,
    context.headings.total,
    context.headings.commission,
    context.headings.net,
  ]);
  const body = rows.map((row) => line([
    row.reference,
    row.propertyName,
    row.unitTypeName,
    row.guest?.fullName ?? '',
    formatIsoDay(row.checkIn),
    formatIsoDay(row.checkOut),
    row.nights.length,
    row.adultCount,
    row.childCount,
    context.stateLabel(row.state),
    formatCdf(row.total, context.locale),
    formatCdf(row.commissionAmount, context.locale),
    formatCdf(row.netAmount, context.locale),
  ]));
  return `\uFEFF${[header, ...body].join('\r\n')}\r\n`;
};

export const todayStamp = (): string => {
  const now = new Date();
  const month = String(now.getMonth() + 1).padStart(2, '0');
  const day = String(now.getDate()).padStart(2, '0');
  return `${now.getFullYear()}-${month}-${day}`;
};

export const downloadCsv = (fileName: string, content: string): void => {
  const blob = new Blob([content], { type: 'text/csv;charset=utf-8' });
  const url = URL.createObjectURL(blob);
  const anchor = document.createElement('a');
  anchor.href = url;
  anchor.download = fileName;
  document.body.appendChild(anchor);
  anchor.click();
  anchor.remove();
  window.setTimeout(() => URL.revokeObjectURL(url), 0);
};
