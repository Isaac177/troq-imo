import { addDays } from '@mungini/utils';
import type { CalendarBlocksView } from './useCalendarBlocks';
import type { CalendarMonthState } from './useCalendarMonth';
import type { CalendarRestrictionsView } from './useCalendarRestrictions';

export const useCalendarDayActions = (month: CalendarMonthState, blocks: CalendarBlocksView, restrictions: CalendarRestrictionsView, canWrite: boolean) => {
  const range = () => month.selectedDay ? { start: month.selectedDay, end: addDays(month.selectedDay, 1) } : null;

  const addBlock = () => {
    const selected = range();
    if (!canWrite || !selected) return;
    month.closeDay();
    blocks.openCreate(selected);
  };

  const addRestriction = () => {
    const selected = range();
    if (!canWrite || !selected) return;
    month.closeDay();
    restrictions.openCreate(selected);
  };

  const removeBlock = (block: { id: string; version: number }) => {
    if (!canWrite) return;
    month.closeDay();
    blocks.askRemove(block);
  };

  const removeRestriction = (restriction: { id: string }) => {
    if (!canWrite) return;
    month.closeDay();
    restrictions.setRemoveTarget(restriction);
  };

  return { canWrite, addBlock, addRestriction, removeBlock, removeRestriction, labels: {
    block: blocks.labels.blockTitle,
    removeBlock: blocks.labels.removeBlock,
    restriction: restrictions.labels.addRestriction,
    removeRestriction: restrictions.labels.removeRestriction,
  } };
};

export type CalendarDayActions = ReturnType<typeof useCalendarDayActions>;
