import { useSearchParams } from 'react-router-dom';

export type CalendarViewMode = 'strip' | 'month';

const VIEW_PARAM = 'view';

export const useCalendarViewMode = () => {
  const [params, setParams] = useSearchParams();
  const mode: CalendarViewMode = params.get(VIEW_PARAM) === 'month' ? 'month' : 'strip';
  const setMode = (value: string) => {
    const next = new URLSearchParams(params);
    next.set(VIEW_PARAM, value === 'month' ? 'month' : 'strip');
    setParams(next, { replace: true });
  };
  return { mode, setMode };
};
