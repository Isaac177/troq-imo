import { ChevronRight } from 'lucide-react';
import { Badge, StatusDot, TonePill } from '@mungini/ui-web';
import type { NotificationRowModel } from '@/types/notifications/notifications';

type Props = { row: NotificationRowModel };

export default function NotificationRow({ row }: Props) {
  const content = (
    <div className="flex min-w-0 flex-1 items-start gap-3">
      <div className="min-w-0 flex-1">
        <div className="flex min-w-0 flex-wrap items-center gap-x-2 gap-y-1">
          <p className={row.unread ? 'min-w-0 break-words text-body font-semibold text-foreground' : 'min-w-0 break-words text-body font-medium text-foreground'}>{row.title}</p>
          <StatusDot tone={row.stateTone} label={row.stateLabel} />
        </div>
        <p className="mt-1 break-words text-label text-muted-foreground">{row.body}</p>
        <div className="mt-2 flex flex-wrap items-center gap-2">
          <Badge variant="outline">{row.typeLabel}</Badge>
          <TonePill tone={row.priorityTone} label={row.priorityLabel} />
          <span className="text-label tabular-nums text-muted-foreground">{row.timestamp}</span>
        </div>
      </div>
      {row.navigable ? <ChevronRight className="mt-1 size-4 shrink-0 text-muted-foreground" aria-hidden="true" /> : null}
    </div>
  );

  if (!row.interactive) {
    return <div className="flex w-full items-start px-3 py-3 md:px-4">{content}</div>;
  }

  return (
    <button
      type="button"
      onClick={row.onOpen}
      className="flex w-full items-start px-3 py-3 text-left transition-colors duration-200 ease-in-out hover:bg-muted/50 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-inset focus-visible:ring-ring md:px-4"
    >
      {content}
    </button>
  );
}
