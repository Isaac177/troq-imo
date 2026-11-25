import { useMemo, useState } from 'react';
import { pad2 } from '@mungini/utils';
import { formatIsoDay } from '@mungini/utils';
import type { SelectOption } from '../form/selectOption';

export type DateTimeValue = { date: string | null; time: string | null };

type Options = {
  value: DateTimeValue;
  onChange: (value: DateTimeValue) => void;
  minuteStep: number;
};

const range = (count: number, step: number): string[] => Array.from({ length: Math.ceil(count / step) }, (_, index) => pad2(index * step));

export const useDateTimePicker = ({ value, onChange, minuteStep }: Options) => {
  const [open, setOpen] = useState(false);
  const [hour, minute] = (value.time ?? '').split(':');
  const hourOptions = useMemo<SelectOption[]>(() => range(24, 1).map((item) => ({ value: item, label: item })), []);
  const minuteOptions = useMemo<SelectOption[]>(() => {
    const items = range(60, minuteStep);
    if (minute && !items.includes(minute)) items.push(minute);
    return items.sort().map((item) => ({ value: item, label: item }));
  }, [minuteStep, minute]);
  const day = formatIsoDay(value.date);
  const time = value.time ?? '';
  return {
    open,
    setOpen,
    hour: hour ?? '',
    minute: minute ?? '',
    hourOptions,
    minuteOptions,
    text: day === '' ? '' : time === '' ? day : `${day} ${time}`,
    selectDay: (iso: string) => onChange({ date: iso, time: value.time ?? '00:00' }),
    selectHour: (next: string) => onChange({ date: value.date, time: `${next}:${minute || '00'}` }),
    selectMinute: (next: string) => onChange({ date: value.date, time: `${hour || '00'}:${next}` }),
    clear: () => {
      onChange({ date: null, time: null });
      setOpen(false);
    },
  };
};
