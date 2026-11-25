import { useMemo, type ChangeEvent } from 'react';
import { useUiCopy } from '../copy/useUiCopy';
import { displayAmount, normalizeAmountText, separatorsOf } from './amountFormat';

type Options = {
  value: string;
  onValueChange: (value: string) => void;
  fractionDigits: number;
};

export const useAmountInput = ({ value, onValueChange, fractionDigits }: Options) => {
  const { locale } = useUiCopy();
  const separators = useMemo(() => separatorsOf(locale), [locale]);
  return {
    text: displayAmount(value, separators),
    change: (event: ChangeEvent<HTMLInputElement>) => onValueChange(normalizeAmountText(event.target.value, separators, fractionDigits)),
  };
};
