import { useState } from 'react';
import { formatIsoDay } from '@mungini/utils';

type Options = {
  value: string | null;
  onChange: (value: string | null) => void;
};

export const useDatePicker = ({ value, onChange }: Options) => {
  const [open, setOpen] = useState(false);
  return {
    open,
    setOpen,
    text: formatIsoDay(value),
    select: (day: string) => {
      onChange(day);
      setOpen(false);
    },
    clear: () => {
      onChange(null);
      setOpen(false);
    },
  };
};
