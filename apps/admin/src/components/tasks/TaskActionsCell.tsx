import type { ReactNode } from 'react';
import { Check, Play, X } from 'lucide-react';
import { Button } from '@mungini/ui-web';
import type { TaskAction } from '@/types/tasks/tasks';

type Props = {
  actions: TaskAction[];
  taskId: string;
  running: { taskId: string; action: TaskAction } | null;
  labels: Record<TaskAction, string>;
  onAction: (action: TaskAction) => void;
};

const icons: Record<TaskAction, ReactNode> = { START: <Play />, COMPLETE: <Check />, CANCEL: <X /> };

export default function TaskActionsCell({ actions, taskId, running, labels, onAction }: Props) {
  return (
    <div className="flex justify-end gap-1">
      {actions.map((action) => (
        <Button
          key={action}
          size="sm"
          variant="outline"
          icon={icons[action]}
          collapseLabel
          aria-label={labels[action]}
          title={labels[action]}
          loading={running?.taskId === taskId && running.action === action}
          disabled={running !== null}
          onClick={() => onAction(action)}
          className={action === 'CANCEL' ? 'text-destructive' : undefined}
        >
          {labels[action]}
        </Button>
      ))}
    </div>
  );
}
