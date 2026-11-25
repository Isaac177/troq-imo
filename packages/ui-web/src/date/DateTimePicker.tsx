import { Popover } from '../overlay/Popover';
import { Button } from '../button/Button';
import { Select } from '../form/Select';
import { useUiCopy } from '../copy/useUiCopy';
import { Calendar } from './Calendar';
import { DateFieldTrigger } from './DateFieldTrigger';
import { useDateTimePicker, type DateTimeValue } from './useDateTimePicker';

type DateTimePickerProps = {
  value: DateTimeValue;
  onChange: (value: DateTimeValue) => void;
  placeholder?: string;
  minDay?: string;
  maxDay?: string;
  today?: string;
  minuteStep?: number;
  clearable?: boolean;
  ariaLabel?: string;
  id?: string;
  disabled?: boolean;
  invalid?: boolean;
  className?: string;
};

export const DateTimePicker = ({ value, onChange, placeholder, minDay, maxDay, today, minuteStep = 5, clearable = false, ariaLabel, id, disabled, invalid, className }: DateTimePickerProps) => {
  const { date, common } = useUiCopy();
  const view = useDateTimePicker({ value, onChange, minuteStep });
  return (
    <Popover
      open={view.open}
      onOpenChange={view.setOpen}
      className="w-[280px] p-3"
      trigger={<DateFieldTrigger id={id} text={view.text} placeholder={placeholder ?? date.pickDateTime} aria-label={ariaLabel} disabled={disabled} invalid={invalid} className={className} />}
    >
      <Calendar selected={value.date} onSelectDay={view.selectDay} minDay={minDay} maxDay={maxDay} today={today} />
      <div className="mt-3 flex items-center gap-2 border-t border-border pt-3">
        <Select options={view.hourOptions} value={view.hour} onValueChange={view.selectHour} ariaLabel={date.hours} placeholder="00" />
        <span className="text-body text-muted-foreground" aria-hidden="true">:</span>
        <Select options={view.minuteOptions} value={view.minute} onValueChange={view.selectMinute} ariaLabel={date.minutes} placeholder="00" />
        {clearable && value.date ? (
          <Button variant="ghost" size="sm" onClick={view.clear} className="ml-auto">
            {common.clear}
          </Button>
        ) : null}
      </div>
    </Popover>
  );
};
