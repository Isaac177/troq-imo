import { useCallback, useEffect, useRef } from 'react';
import { dismissToast } from './toastStore';

export const useToastTimer = (id: string, durationMs: number) => {
  const timer = useRef<number | null>(null);
  const clear = useCallback(() => {
    if (timer.current !== null) window.clearTimeout(timer.current);
    timer.current = null;
  }, []);
  const start = useCallback(() => {
    clear();
    timer.current = window.setTimeout(() => dismissToast(id), durationMs);
  }, [clear, id, durationMs]);
  useEffect(() => {
    start();
    return clear;
  }, [start, clear]);
  return { pause: clear, resume: start, dismiss: () => dismissToast(id) };
};
