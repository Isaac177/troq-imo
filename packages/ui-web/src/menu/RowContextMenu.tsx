import { Fragment, type ReactElement } from 'react';
import * as ContextPrimitive from '@radix-ui/react-context-menu';
import type { ActionItem } from './actionItem';
import { menuContentClasses, menuRowClasses, menuSeparatorClasses } from './menuClasses';

type RowContextMenuProps = {
  items: ActionItem[];
  children: ReactElement;
};

export const RowContextMenu = ({ items, children }: RowContextMenuProps) => (
  <ContextPrimitive.Root>
    <ContextPrimitive.Trigger asChild>{children}</ContextPrimitive.Trigger>
    <ContextPrimitive.Portal>
      <ContextPrimitive.Content collisionPadding={8} className={menuContentClasses}>
        {items.map((item) => (
          <Fragment key={item.key}>
            {item.separatorBefore ? <ContextPrimitive.Separator className={menuSeparatorClasses} /> : null}
            <ContextPrimitive.Item disabled={item.disabled} onSelect={item.onSelect} className={menuRowClasses(item.destructive)}>
              {item.icon}
              {item.label}
            </ContextPrimitive.Item>
          </Fragment>
        ))}
      </ContextPrimitive.Content>
    </ContextPrimitive.Portal>
  </ContextPrimitive.Root>
);
