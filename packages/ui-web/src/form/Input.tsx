import type { ComponentProps, ReactNode } from 'react';
import { cn } from '../lib/cn';
import { controlClasses } from '../lib/fieldClasses';

type InputType = 'text' | 'email' | 'password' | 'number' | 'search' | 'url' | 'time' | 'date';

export type InputProps = Omit<ComponentProps<'input'>, 'type' | 'size'> & {
  type?: InputType;
  leadingIcon?: ReactNode;
  trailing?: ReactNode;
  invalid?: boolean;
  wrapperClassName?: string;
};

export const Input = ({ type = 'text', leadingIcon, trailing, invalid, className, wrapperClassName, ...rest }: InputProps) => {
  const field = (
    <input
      {...rest}
      type={type}
      aria-invalid={invalid || undefined}
      className={cn(controlClasses, leadingIcon && 'pl-9', trailing && 'pr-9', className)}
    />
  );
  if (!leadingIcon && !trailing) return field;
  return (
    <div className={cn('relative w-full', wrapperClassName)}>
      {leadingIcon ? (
        <span className="pointer-events-none absolute left-3 top-1/2 -translate-y-1/2 text-muted-foreground [&_svg]:size-4" aria-hidden="true">
          {leadingIcon}
        </span>
      ) : null}
      {field}
      {trailing ? <span className="absolute right-1.5 top-1/2 flex -translate-y-1/2 items-center">{trailing}</span> : null}
    </div>
  );
};
