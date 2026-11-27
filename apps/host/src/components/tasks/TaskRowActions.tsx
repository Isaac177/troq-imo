import type { ReactNode } from 'react';
import { Check, Play, UserMinus, UserPlus, X } from 'lucide-react';
import { Button } from '@mungini/ui-web';
import type { TaskAction, TaskActionItem } from '@/types/tasks/tasks';

const ICONS: Record<TaskAction, ReactNode> = {
  START: <Play />,
  COMPLETE: <Check />,
  CANCEL: <X />,
  ASSIGN_ME: <UserPlus />,
  UNASSIGN: <UserMinus />,
};

type Props = {
  actions: TaskActionItem[];
  pending: TaskAction | null;
};

export default function TaskRowActions({ actions, pending }: Props) {
  return (
    <div className="flex items-center justify-end gap-1">
      {actions.map((item) => (
        <Button
          key={item.action}
          variant="outline"
          size="sm"
          icon={ICONS[item.action]}
          collapseLabel
          aria-label={item.label}
          title={item.label}
          loading={pending === item.action}
          disabled={pending !== null}
          onClick={item.run}
        >
          {item.label}
        </Button>
      ))}
    </div>
  );
}
