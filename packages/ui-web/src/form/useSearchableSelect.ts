import { useCallback, useMemo, useState } from 'react';
import type { SelectOption } from './selectOption';

type Options = {
  options: SelectOption[];
  value: string;
  selectedLabel?: string;
  onValueChange: (value: string) => void;
};

export const useSearchableSelect = ({ options, value, selectedLabel, onValueChange }: Options) => {
  const [open, setOpen] = useState(false);
  const label = useMemo(() => selectedLabel ?? options.find((option) => option.value === value)?.label ?? '', [options, value, selectedLabel]);
  const commandOptions = useMemo(() => options.filter((option) => !option.disabled).map(({ value: id, label: text }) => ({ value: id, label: text })), [options]);
  const select = useCallback(
    (next: string) => {
      onValueChange(next);
      setOpen(false);
    },
    [onValueChange],
  );
  return { open, setOpen, label, commandOptions, select };
};
