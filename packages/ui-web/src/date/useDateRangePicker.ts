import { useState } from 'react';
import { formatIsoDay } from '@mungini/utils';
import type { CalendarRange } from './Calendar';
import { nextRange } from './rangeSelection';

type Options = {
  value: CalendarRange;
  onChange: (value: CalendarRange) => void;
};

export const useDateRangePicker = ({ value, onChange }: Options) => {
  const [open, setOpen] = useState(false);
  const start = formatIsoDay(value.start);
  const end = formatIsoDay(value.end);
  return {
    open,
    setOpen,
    text: start === '' ? '' : end === '' ? start : `${start} - ${end}`,
    select: (day: string) => {
      const result = nextRange(value, day);
      onChange(result.range);
      if (result.complete) setOpen(false);
    },
    clear: () => {
      onChange({ start: null, end: null });
      setOpen(false);
    },
  };
};
