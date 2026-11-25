import { ToastItem } from './ToastItem';
import { useToasts } from './useToasts';

export const ToastViewport = () => {
  const toasts = useToasts();
  return (
    <div className="pointer-events-none fixed inset-x-0 bottom-0 z-[100] flex flex-col items-center gap-2 p-3 sm:inset-x-auto sm:right-0 sm:w-[380px] sm:items-stretch sm:p-4">
      {toasts.map((item) => (
        <ToastItem key={item.id} {...item} />
      ))}
    </div>
  );
};
