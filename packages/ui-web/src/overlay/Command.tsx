import type { ReactNode } from 'react';
import { Command as CommandPrimitive } from 'cmdk';
import { Check, Loader2, Search } from 'lucide-react';
import { cn } from '../lib/cn';
import { useUiCopy } from '../copy/useUiCopy';

export type CommandOption = {
  value: string;
  label: string;
  keywords?: string[];
  detail?: string;
  leading?: ReactNode;
};

type CommandProps = {
  options: CommandOption[];
  selectedValue?: string | null;
  onSelect: (value: string) => void;
  searchPlaceholder: string;
  emptyLabel?: string;
  search?: string;
  onSearchChange?: (search: string) => void;
  loading?: boolean;
  className?: string;
};

export const Command = ({ options, selectedValue, onSelect, searchPlaceholder, emptyLabel, search, onSearchChange, loading, className }: CommandProps) => {
  const { common } = useUiCopy();
  return (
    <CommandPrimitive shouldFilter={onSearchChange === undefined} className={cn('flex w-full flex-col overflow-hidden', className)}>
      <div className="flex h-9 items-center gap-2 border-b border-border px-3">
        <Search className="size-4 shrink-0 text-muted-foreground" aria-hidden="true" />
        <CommandPrimitive.Input
          value={search}
          onValueChange={onSearchChange}
          placeholder={searchPlaceholder}
          aria-label={searchPlaceholder}
          className="h-full w-full bg-transparent text-body text-foreground outline-none placeholder:text-muted-foreground"
        />
        {loading ? <Loader2 className="size-4 shrink-0 animate-spin text-muted-foreground" aria-label={common.loading} /> : null}
      </div>
      <CommandPrimitive.List className="max-h-[260px] overflow-y-auto p-1">
        <CommandPrimitive.Empty className="px-2 py-6 text-center text-body text-muted-foreground">{emptyLabel ?? common.noResults}</CommandPrimitive.Empty>
        {options.map((option) => (
          <CommandPrimitive.Item
            key={option.value}
            value={option.value}
            keywords={[option.label, ...(option.keywords ?? [])]}
            onSelect={() => onSelect(option.value)}
            className="flex min-h-9 cursor-default select-none items-center gap-2 rounded-md px-2 text-body text-foreground outline-none transition-colors duration-200 ease-in-out data-[selected=true]:bg-accent"
          >
            {option.leading ? <span className="shrink-0" aria-hidden="true">{option.leading}</span> : null}
            <span className="min-w-0 flex-1 truncate">{option.label}</span>
            {option.detail ? <span className="shrink-0 text-label tabular-nums text-muted-foreground">{option.detail}</span> : null}
            <Check className={cn('size-4 shrink-0 transition-opacity duration-200 ease-in-out', option.value === selectedValue ? 'opacity-100' : 'opacity-0')} aria-hidden="true" />
          </CommandPrimitive.Item>
        ))}
      </CommandPrimitive.List>
    </CommandPrimitive>
  );
};
