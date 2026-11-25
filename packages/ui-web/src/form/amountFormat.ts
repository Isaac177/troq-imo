export type NumberSeparators = { group: string; decimal: string };

export const separatorsOf = (locale: string): NumberSeparators => {
  const parts = new Intl.NumberFormat(locale.startsWith('en') ? 'en-US' : 'fr-CD').formatToParts(1234567.8);
  return {
    group: parts.find((part) => part.type === 'group')?.value ?? ' ',
    decimal: parts.find((part) => part.type === 'decimal')?.value ?? ',',
  };
};

export const normalizeAmountText = (text: string, separators: NumberSeparators, fractionDigits: number): string => {
  const decimalChars = new Set(['.', separators.decimal]);
  decimalChars.delete(separators.group);
  const decimalAt = fractionDigits > 0 ? Array.from(text).findIndex((char) => decimalChars.has(char)) : -1;
  const integerSource = decimalAt === -1 ? text : text.slice(0, decimalAt);
  const integer = integerSource.replace(/\D/g, '').replace(/^0+(?=\d)/, '');
  if (decimalAt === -1) return integer;
  const fraction = text.slice(decimalAt + 1).replace(/\D/g, '').slice(0, fractionDigits);
  return `${integer === '' ? '0' : integer}.${fraction}`;
};

export const displayAmount = (value: string, separators: NumberSeparators): string => {
  if (value === '') return '';
  const [integer = '', fraction] = value.split('.');
  const grouped = integer.replace(/\B(?=(\d{3})+(?!\d))/g, separators.group);
  return fraction === undefined ? grouped : `${grouped}${separators.decimal}${fraction}`;
};
