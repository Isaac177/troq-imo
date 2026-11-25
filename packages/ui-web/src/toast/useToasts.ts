import { useSyncExternalStore } from 'react';
import { getToasts, subscribeToasts, type ToastData } from './toastStore';

export const useToasts = (): ToastData[] => useSyncExternalStore(subscribeToasts, getToasts, getToasts);
