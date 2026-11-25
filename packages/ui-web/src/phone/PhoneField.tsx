import { Globe } from 'lucide-react';
import { cn } from '../lib/cn';
import { controlClasses } from '../lib/fieldClasses';
import { Input } from '../form/Input';
import { Command } from '../overlay/Command';
import { Popover } from '../overlay/Popover';
import { useUiCopy } from '../copy/useUiCopy';
import { usePhoneField } from './usePhoneField';

type PhoneFieldProps = {
  value: string;
  onChange: (value: string) => void;
  fallbackCountry?: string;
  detectCountry?: () => Promise<string | null>;
  placeholder?: string;
  id?: string;
  disabled?: boolean;
  invalid?: boolean;
  className?: string;
};

export const PhoneField = ({ value, onChange, fallbackCountry = 'CD', detectCountry, placeholder, id, disabled, invalid, className }: PhoneFieldProps) => {
  const { phone } = useUiCopy();
  const view = usePhoneField({ value, onChange, fallbackCountry, detectCountry });
  return (
    <div className={cn('flex w-full items-center gap-2', className)}>
      <Popover
        open={view.open}
        onOpenChange={view.setOpen}
        className="w-[280px] p-0"
        trigger={
          <button type="button" disabled={disabled} aria-label={phone.country} className={cn(controlClasses, 'flex w-12 shrink-0 items-center justify-center px-0')}>
            {view.flag ? <span aria-hidden="true">{view.flag}</span> : <Globe className="size-4 text-muted-foreground" aria-hidden="true" />}
          </button>
        }
      >
        <Command options={view.options} selectedValue={view.countryCode} onSelect={view.selectCountry} searchPlaceholder={phone.countrySearch} emptyLabel={phone.countryEmpty} />
      </Popover>
      <Input id={id} type="text" inputMode="tel" autoComplete="tel" disabled={disabled} invalid={invalid} value={view.text} onChange={view.changeInput} placeholder={placeholder} />
    </div>
  );
};
