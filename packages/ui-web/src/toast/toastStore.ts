export type ToastTone = 'neutral' | 'success' | 'warning' | 'danger';

export type ToastInput = {
  title: string;
  description?: string;
  tone?: ToastTone;
  durationMs?: number;
};

export type ToastData = {
  id: string;
  title: string;
  description?: string;
  tone: ToastTone;
  durationMs: number;
};

const DEFAULT_DURATION_MS = 4500;
const MAX_VISIBLE = 4;

type Listener = () => void;

let items: ToastData[] = [];
let counter = 0;
const listeners = new Set<Listener>();

const publish = (next: ToastData[]): void => {
  items = next;
  listeners.forEach((listener) => listener());
};

export const subscribeToasts = (listener: Listener): (() => void) => {
  listeners.add(listener);
  return () => {
    listeners.delete(listener);
  };
};

export const getToasts = (): ToastData[] => items;

export const dismissToast = (id: string): void => publish(items.filter((item) => item.id !== id));

export const toast = ({ title, description, tone = 'neutral', durationMs = DEFAULT_DURATION_MS }: ToastInput): string => {
  counter += 1;
  const id = `toast-${counter}`;
  publish([...items, { id, title, description, tone, durationMs }].slice(-MAX_VISIBLE));
  return id;
};
