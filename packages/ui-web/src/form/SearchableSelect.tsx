import { ChevronsUpDown } from 'lucide-react';
import { cn } from '../lib/cn';
import { controlClasses } from '../lib/fieldClasses';
import { Command } from '../overlay/Command';
import { Popover } from '../overlay/Popover';
import type { SelectOption } from './selectOption';
import { useSearchableSelect } from './useSearchableSelect';

type SearchableSelectProps = {
  options: SelectOption[];
  value: string;
  onValueChange: (value: string) => void;
  placeholder: string;
  searchPlaceholder: string;
  emptyLabel?: string;
  selectedLabel?: string;
  search?: string;
  onSearchChange?: (search: string) => void;
  loading?: boolean;
  ariaLabel?: string;
  id?: string;
  disabled?: boolean;
  invalid?: boolean;
  className?: string;
};

export const SearchableSelect = ({
  options,
  value,
  onValueChange,
  placeholder,
  searchPlaceholder,
  emptyLabel,
  selectedLabel,
  search,
  onSearchChange,
  loading,
  ariaLabel,
  id,
  disabled,
  invalid,
  className,
}: SearchableSelectProps) => {
  const view = useSearchableSelect({ options, value, selectedLabel, onValueChange });
  return (
    <Popover
      open={view.open}
      onOpenChange={view.setOpen}
      className="w-(--radix-popover-trigger-width) min-w-[220px] p-0"
      trigger={
        <button
          id={id}
          type="button"
          role="combobox"
          aria-expanded={view.open}
          aria-label={ariaLabel}
          aria-invalid={invalid || undefined}
          disabled={disabled}
          className={cn(controlClasses, 'flex items-center justify-between gap-2 text-left', className)}
        >
          <span className={cn('truncate', view.label === '' && 'text-muted-foreground')}>{view.label === '' ? placeholder : view.label}</span>
          <ChevronsUpDown className="size-4 shrink-0 text-muted-foreground" aria-hidden="true" />
        </button>
      }
    >
      <Command
        options={view.commandOptions}
        selectedValue={value}
        onSelect={view.select}
        searchPlaceholder={searchPlaceholder}
        emptyLabel={emptyLabel}
        search={search}
        onSearchChange={onSearchChange}
        loading={loading}
      />
    </Popover>
  );
};
