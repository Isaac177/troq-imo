import { Popover } from '../overlay/Popover';
import { Button } from '../button/Button';
import { useUiCopy } from '../copy/useUiCopy';
import { Calendar } from './Calendar';
import { DateFieldTrigger } from './DateFieldTrigger';
import { useDatePicker } from './useDatePicker';

type DatePickerProps = {
  value: string | null;
  onChange: (value: string | null) => void;
  placeholder?: string;
  minDay?: string;
  maxDay?: string;
  today?: string;
  clearable?: boolean;
  ariaLabel?: string;
  id?: string;
  disabled?: boolean;
  invalid?: boolean;
  className?: string;
};

export const DatePicker = ({ value, onChange, placeholder, minDay, maxDay, today, clearable = false, ariaLabel, id, disabled, invalid, className }: DatePickerProps) => {
  const { date, common } = useUiCopy();
  const view = useDatePicker({ value, onChange });
  return (
    <Popover
      open={view.open}
      onOpenChange={view.setOpen}
      className="w-[280px] p-3"
      trigger={<DateFieldTrigger id={id} text={view.text} placeholder={placeholder ?? date.pick} aria-label={ariaLabel} disabled={disabled} invalid={invalid} className={className} />}
    >
      <Calendar selected={value} onSelectDay={view.select} minDay={minDay} maxDay={maxDay} today={today} />
      {clearable && value ? (
        <div className="mt-2 flex justify-end">
          <Button variant="ghost" size="sm" onClick={view.clear}>
            {common.clear}
          </Button>
        </div>
      ) : null}
    </Popover>
  );
};
