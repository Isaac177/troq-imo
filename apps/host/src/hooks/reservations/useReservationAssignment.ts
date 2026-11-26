import { useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { assignUnit, listUnitOptions, moveAssignment } from '@/services/reservations/reservations.api';
import type { UnitOption } from '@/types/reservations/assignment';

const KNOWN_ERRORS = ['ITEM_NOT_FOUND', 'ASSIGNMENT_STATE_INVALID', 'UNIT_UNAVAILABLE', 'UNIT_INCOMPATIBLE', 'ASSIGNMENT_ALREADY_HAS_UNIT', 'ASSIGNMENT_SAME_UNIT', 'ASSIGNMENT_MOVE_DATE_INVALID', 'UNIT_TURNOVER_BUFFER', 'ASSIGNMENT_CHANGED', 'ASSIGNMENT_INACTIVE', 'FORBIDDEN'];

export const useReservationAssignment = (input: { itemId: string; state: string; assignmentId: string | null; checkIn: string }, onChanged: () => void) => {
  const { t } = useTranslation('hostReservations');
  const translate = t as unknown as (key: string) => string;
  const { client } = useSession();
  const [open, setOpen] = useState(false);
  const [options, setOptions] = useState<UnitOption[]>([]);
  const [loading, setLoading] = useState(false);
  const [unitId, setUnitId] = useState('');
  const [fromDate, setFromDate] = useState(input.checkIn);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [notice, setNotice] = useState<string | null>(null);

  const message = (code: string | null) => (code && KNOWN_ERRORS.includes(code) ? translate(`roomErrors.${code}`) : translate('roomErrors.generic'));
  const canAssign = input.state === 'CONFIRMED' || input.state === 'CHECKED_IN';

  const openDialog = async () => {
    setOpen(true);
    setError(null);
    setNotice(null);
    setFromDate(input.checkIn);
    setLoading(true);
    const result = await listUnitOptions(client, input.itemId);
    setLoading(false);
    if (result.error) {
      setError(message(result.error));
      setOptions([]);
      return;
    }
    setOptions(result.data ?? []);
    setUnitId(result.data?.[0]?.id ?? '');
  };

  const submit = async () => {
    if (!unitId) return;
    setBusy(true);
    setError(null);
    const result = input.assignmentId
      ? await moveAssignment(client, input.assignmentId, unitId, fromDate)
      : await assignUnit(client, input.itemId, unitId);
    setBusy(false);
    if (result.error) {
      setError(message(result.error));
      return;
    }
    setOpen(false);
    setNotice(translate('roomSaved'));
    onChanged();
  };

  return {
    open, setOpen, openDialog, options, loading, unitId, setUnitId, fromDate, setFromDate, busy, error, notice, canAssign, submit,
    optionOptions: options.map((option) => ({ value: option.id, label: option.localCode })),
    labels: {
      roomAssignment: translate('roomAssignment'), assignedRoom: translate('assignedRoom'), noRoomAssigned: translate('noRoomAssigned'),
      assignRoom: translate('assignRoom'), changeRoom: translate('changeRoom'), chooseRoom: translate('chooseRoom'), moveFrom: translate('moveFrom'),
      noRoomOptions: translate('noRoomOptions'), close: translate('close'), dismiss: translate('dismiss'), roomSaved: translate('roomSaved'),
      error: translate('roomErrors.generic'),
    },
  };
};

export type ReservationAssignmentView = ReturnType<typeof useReservationAssignment>;
