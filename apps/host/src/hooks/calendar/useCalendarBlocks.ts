import { useEffect, useState } from 'react';
import type { CalendarRange } from '@mungini/ui-web';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { createBlock, removeBlock } from '@/services/calendar/calendar.api';
import type { PropertyOption } from '@/types/calendar/calendar';

const KNOWN_ERRORS = ['NO_NIGHTS_AVAILABLE', 'BLOCK_NOT_FOUND', 'FORBIDDEN', 'PROPERTY_NOT_FOUND', 'BLOCK_REASON_REQUIRED'];

type RemoveTarget = { id: string; version: number };

export const useCalendarBlocks = (propertyId: string, unitOptions: PropertyOption[], onChanged: () => void) => {
  const { t } = useTranslation('hostCalendar');
  const translate = t as unknown as (key: string) => string;
  const { client } = useSession();
  const [open, setOpen] = useState(false);
  const [unitTypeId, setUnitTypeId] = useState('');
  const [range, setRange] = useState<CalendarRange>({ start: null, end: null });
  const [reason, setReason] = useState('');
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [notice, setNotice] = useState<string | null>(null);
  const [removeTarget, setRemoveTarget] = useState<RemoveTarget | null>(null);
  const [removeBusy, setRemoveBusy] = useState(false);

  const message = (code: string | null) => (code && KNOWN_ERRORS.includes(code) ? translate(`errors.${code}`) : translate('errors.generic'));

  useEffect(() => {
    setRange({ start: null, end: null });
    setReason('');
    setError(null);
    setNotice(null);
  }, [propertyId]);

  useEffect(() => {
    if (!unitOptions.some((option) => option.value === unitTypeId)) setUnitTypeId(unitOptions[0]?.value ?? '');
  }, [unitOptions, unitTypeId]);

  const submit = async () => {
    if (!propertyId || !unitTypeId || !range.start || !range.end) return;
    setBusy(true);
    setError(null);
    setNotice(null);
    const result = await createBlock(client, { propertyId, unitTypeId, startDate: range.start, endDate: range.end, reason });
    setBusy(false);
    if (result.error) {
      setError(message(result.error));
      return;
    }
    setOpen(false);
    setReason('');
    setRange({ start: null, end: null });
    setNotice(translate('blockDone'));
    onChanged();
  };

  const confirmRemove = async () => {
    if (!removeTarget) return;
    setRemoveBusy(true);
    setError(null);
    const result = await removeBlock(client, removeTarget.id, removeTarget.version);
    setRemoveBusy(false);
    if (result.error) {
      setError(message(result.error));
      return;
    }
    setRemoveTarget(null);
    setNotice(translate('blockRemoved'));
    onChanged();
  };

  const askRemove = (block: RemoveTarget) => setRemoveTarget(block);

  return {
    open,
    setOpen,
    unitTypeId,
    setUnitTypeId,
    range,
    setRange,
    reason,
    setReason,
    busy,
    error,
    notice,
    submit,
    removeTarget,
    setRemoveTarget,
    removeBusy,
    askRemove,
    confirmRemove,
    labels: {
      blockTitle: translate('blockTitle'),
      blockBody: translate('blockBody'),
      blockUnit: translate('blockUnit'),
      blockReason: translate('blockReason'),
      blockReasonPlaceholder: translate('blockReasonPlaceholder'),
      blockSubmit: translate('blockSubmit'),
      confirmRemoveTitle: translate('confirmRemoveTitle'),
      confirmRemoveBody: translate('confirmRemoveBody'),
      removeBlock: translate('removeBlock'),
      close: translate('close'),
      dismiss: translate('dismiss'),
      blockDone: translate('blockDone'),
      blockRemoved: translate('blockRemoved'),
    },
  };
};

export type CalendarBlocksView = ReturnType<typeof useCalendarBlocks>;
