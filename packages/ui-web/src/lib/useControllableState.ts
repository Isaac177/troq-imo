import { useCallback, useState } from 'react';

type ControllableOptions<T> = {
  value: T | undefined;
  defaultValue: T;
  onChange?: (value: T) => void;
};

export const useControllableState = <T,>({ value, defaultValue, onChange }: ControllableOptions<T>): [T, (next: T) => void] => {
  const [inner, setInner] = useState<T>(defaultValue);
  const controlled = value !== undefined;
  const set = useCallback(
    (next: T) => {
      if (!controlled) setInner(next);
      onChange?.(next);
    },
    [controlled, onChange],
  );
  return [controlled ? value : inner, set];
};
