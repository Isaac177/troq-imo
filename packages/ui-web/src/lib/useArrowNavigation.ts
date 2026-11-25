import { useCallback, type KeyboardEvent } from 'react';

type ArrowNavigationOptions = {
  values: string[];
  value: string;
  onChange: (value: string) => void;
  selector: string;
};

export const useArrowNavigation = ({ values, value, onChange, selector }: ArrowNavigationOptions) =>
  useCallback(
    (event: KeyboardEvent<HTMLElement>) => {
      const forward = event.key === 'ArrowRight' || event.key === 'ArrowDown';
      const backward = event.key === 'ArrowLeft' || event.key === 'ArrowUp';
      if (!forward && !backward && event.key !== 'Home' && event.key !== 'End') return;
      event.preventDefault();
      const current = Math.max(values.indexOf(value), 0);
      const last = values.length - 1;
      const next = event.key === 'Home' ? 0 : event.key === 'End' ? last : forward ? (current + 1) % values.length : (current - 1 + values.length) % values.length;
      const nextValue = values[next];
      if (nextValue === undefined) return;
      onChange(nextValue);
      const items = event.currentTarget.querySelectorAll<HTMLElement>(selector);
      items[next]?.focus();
    },
    [values, value, onChange, selector],
  );
