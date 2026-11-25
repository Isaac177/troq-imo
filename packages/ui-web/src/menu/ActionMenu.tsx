import { Fragment, type ReactNode } from 'react';
import * as DropdownPrimitive from '@radix-ui/react-dropdown-menu';
import { MoreHorizontal } from 'lucide-react';
import { IconTooltipButton } from '../button/IconTooltipButton';
import { useUiCopy } from '../copy/useUiCopy';
import type { ActionItem } from './actionItem';
import { menuContentClasses, menuRowClasses, menuSeparatorClasses } from './menuClasses';

type ActionMenuProps = {
  items: ActionItem[];
  trigger?: ReactNode;
  triggerLabel?: string;
  align?: 'start' | 'center' | 'end';
};

export const ActionMenu = ({ items, trigger, triggerLabel, align = 'end' }: ActionMenuProps) => {
  const { common } = useUiCopy();
  return (
    <DropdownPrimitive.Root>
      <DropdownPrimitive.Trigger asChild>{trigger ?? <IconTooltipButton icon={<MoreHorizontal />} label={triggerLabel ?? common.more} />}</DropdownPrimitive.Trigger>
      <DropdownPrimitive.Portal>
        <DropdownPrimitive.Content align={align} sideOffset={4} collisionPadding={8} className={menuContentClasses}>
          {items.map((item) => (
            <Fragment key={item.key}>
              {item.separatorBefore ? <DropdownPrimitive.Separator className={menuSeparatorClasses} /> : null}
              <DropdownPrimitive.Item disabled={item.disabled} onSelect={item.onSelect} className={menuRowClasses(item.destructive)}>
                {item.icon}
                {item.label}
              </DropdownPrimitive.Item>
            </Fragment>
          ))}
        </DropdownPrimitive.Content>
      </DropdownPrimitive.Portal>
    </DropdownPrimitive.Root>
  );
};
