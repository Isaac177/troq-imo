const formatters = new Map<string, Intl.NumberFormat>();

const formatterFor = (locale: string, fractionDigits: number): Intl.NumberFormat => {
  const tag = locale.startsWith('en') ? 'en-US' : 'fr-CD';
  const key = `${tag}:${fractionDigits}`;
  const cached = formatters.get(key);
  if (cached) return cached;
  const created = new Intl.NumberFormat(tag, { minimumFractionDigits: fractionDigits, maximumFractionDigits: fractionDigits });
  formatters.set(key, created);
  return created;
};

export const CDF_CODE = 'CDF';

export const formatCdf = (amount: string | number | null | undefined, locale: string, fractionDigits = 2): string => {
  if (amount === null || amount === undefined || amount === '') return '';
  const value = Number(amount);
  if (!Number.isFinite(value)) return '';
  return `${formatterFor(locale, fractionDigits).format(value)} ${CDF_CODE}`;
};
