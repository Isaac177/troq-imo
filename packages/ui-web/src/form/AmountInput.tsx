import { cn } from '../lib/cn';
import { CDF_CODE } from '@mungini/utils';
import { Input } from './Input';
import { useAmountInput } from './useAmountInput';

type AmountInputProps = {
  value: string;
  onValueChange: (value: string) => void;
  fractionDigits?: number;
  placeholder?: string;
  id?: string;
  ariaLabel?: string;
  disabled?: boolean;
  invalid?: boolean;
  className?: string;
};

export const AmountInput = ({ value, onValueChange, fractionDigits = 0, placeholder, id, ariaLabel, disabled, invalid, className }: AmountInputProps) => {
  const view = useAmountInput({ value, onValueChange, fractionDigits });
  return (
    <Input
      id={id}
      type="text"
      inputMode={fractionDigits > 0 ? 'decimal' : 'numeric'}
      autoComplete="off"
      aria-label={ariaLabel}
      value={view.text}
      onChange={view.change}
      placeholder={placeholder}
      disabled={disabled}
      invalid={invalid}
      className={cn('pr-14 text-right tabular-nums', className)}
      trailing={<span className="pointer-events-none pr-2 text-label text-muted-foreground">{CDF_CODE}</span>}
    />
  );
};
