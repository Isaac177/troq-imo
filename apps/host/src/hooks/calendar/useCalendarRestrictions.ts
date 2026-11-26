import { useEffect, useState } from 'react';
import type { CalendarRange } from '@mungini/ui-web';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { setRestrictions } from '@/services/calendar/calendar.api';
import type { PropertyOption } from '@/types/calendar/calendar';

const NUMERIC_KINDS = ['MIN_NIGHTS', 'MAX_NIGHTS', 'ADVANCE_NOTICE_DAYS', 'MAX_HORIZON_DAYS', 'TURNOVER_BUFFER_NIGHTS'];
const KINDS = [...NUMERIC_KINDS, 'CLOSED_TO_ARRIVAL', 'CLOSED_TO_DEPARTURE'];
const KNOWN_ERRORS = ['RESTRICTION_KIND_REQUIRED', 'RESTRICTION_VALUE_INVALID', 'RESTRICTION_WINDOW_INVALID', 'RESTRICTION_NOT_FOUND', 'RESTRICTION_BASIS_INVALID', 'UNIT_TYPE_NOT_FOUND', 'BULK_SIZE_INVALID', 'FORBIDDEN'];

export const useCalendarRestrictions = (propertyId: string, unitOptions: PropertyOption[], onChanged: () => void) => {
  const { t } = useTranslation('hostCalendar');
  const translate = t as unknown as (key: string) => string;
  const { client } = useSession();
  const [open, setOpen] = useState(false);
  const [kind, setKind] = useState('MIN_NIGHTS');
  const [basis, setBasis] = useState('STAY_THROUGH');
  const [value, setValue] = useState('2');
  const [unitTypeId, setUnitTypeId] = useState('all');
  const [range, setRange] = useState<CalendarRange>({ start: null, end: null });
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [notice, setNotice] = useState<string | null>(null);
  const [removeTarget, setRemoveTarget] = useState<{ id: string } | null>(null);
  const numeric = NUMERIC_KINDS.includes(kind);

  const message = (code: string | null) => (code && KNOWN_ERRORS.includes(code) ? translate(`errors.${code}`) : translate('errors.generic'));

  useEffect(() => {
    setOpen(false);
    setRange({ start: null, end: null });
    setNotice(null);
    setError(null);
    setRemoveTarget(null);
  }, [propertyId]);

  useEffect(() => {
    if (kind !== 'MIN_NIGHTS' && basis === 'ARRIVAL') setBasis('STAY_THROUGH');
  }, [kind, basis]);

  const openCreate = () => {
    setKind('MIN_NIGHTS');
    setBasis('STAY_THROUGH');
    setValue('2');
    setUnitTypeId('all');
    setRange({ start: null, end: null });
    setError(null);
    setNotice(null);
    setOpen(true);
  };

  const submit = async () => {
    if (!propertyId) return;
    setBusy(true);
    setError(null);
    setNotice(null);
    const result = await setRestrictions(client, propertyId, [{
      action: 'ADD',
      kind,
      basis,
      value: numeric ? Number(value) : null,
      startDate: range.start,
      endDate: range.end,
      unitTypeId: unitTypeId === 'all' ? null : unitTypeId,
    }]);
    setBusy(false);
    if (result.error) {
      setError(message(result.error));
      return;
    }
    setOpen(false);
    setNotice(translate('restrictionSaved'));
    onChanged();
  };

  const confirmRemove = async () => {
    if (!propertyId || !removeTarget) return;
    setBusy(true);
    setError(null);
    const result = await setRestrictions(client, propertyId, [{ action: 'REMOVE', restrictionId: removeTarget.id }]);
    setBusy(false);
    if (result.error) {
      setError(message(result.error));
      setRemoveTarget(null);
      return;
    }
    setRemoveTarget(null);
    setNotice(translate('restrictionRemoved'));
    onChanged();
  };

  return {
    open, setOpen, openCreate, submit, kind, setKind, basis, setBasis, value, setValue,
    unitTypeId, setUnitTypeId, range, setRange, busy, error, notice,
    removeTarget, setRemoveTarget, confirmRemove, numeric,
    kindOptions: KINDS.map((item) => ({ value: item, label: translate(`restrictionKinds.${item}`) })),
    basisOptions: (kind === 'MIN_NIGHTS' ? ['STAY_THROUGH', 'ARRIVAL'] : ['STAY_THROUGH']).map((item) => ({ value: item, label: translate(`bases.${item}`) })),
    unitOptions: [{ value: 'all', label: translate('allUnits') }, ...unitOptions],
    labels: {
      addRestriction: translate('addRestriction'), removeRestriction: translate('removeRestriction'), restrictionKind: translate('restrictionKind'),
      restrictionBasis: translate('restrictionBasis'), restrictionValue: translate('restrictionValue'), restrictionUnit: translate('restrictionUnit'),
      confirmRemoveTitle: translate('confirmRemoveRestrictionTitle'), confirmRemoveBody: translate('confirmRemoveRestrictionBody'),
      close: translate('close'), dismiss: translate('dismiss'), error: translate('errors.generic'),
    },
  };
};

export type CalendarRestrictionsView = ReturnType<typeof useCalendarRestrictions>;
