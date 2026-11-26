import { ChevronRight } from 'lucide-react';
import { StatusDot } from '@mungini/ui-web';
import type { NotificationRowView } from '@/types/notifications/notifications';

type Props = {
  row: NotificationRowView;
};

export const NotificationRow = ({ row }: Props) => {
  if (!row.interactive) {
    return (
      <li className="border-b border-border px-4 py-3 last:border-0">
        <span className="flex w-full items-start gap-3">
          <span className="flex min-w-0 flex-1 flex-col gap-1">
            <span className="flex flex-wrap items-center gap-x-3 gap-y-1">
              <StatusDot tone={row.tone} label={row.readLabel} />
              <span className="text-label text-muted-foreground">{row.typeLabel}</span>
              <span className="text-label text-muted-foreground">{row.priorityLabel}</span>
            </span>
            <span className="break-words text-body text-foreground">{row.title}</span>
            <span className="break-words text-body text-muted-foreground">{row.body}</span>
            <span className="text-label tabular-nums text-muted-foreground">{row.timestamp}</span>
          </span>
        </span>
      </li>
    );
  }

  return (
    <li className="border-b border-border last:border-0">
      <button
        type="button"
        onClick={row.open}
        className="flex w-full items-start gap-3 px-4 py-3 text-left transition-colors duration-200 ease-in-out hover:bg-muted/40 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-inset focus-visible:ring-ring"
      >
        <span className="flex min-w-0 flex-1 flex-col gap-1">
          <span className="flex flex-wrap items-center gap-x-3 gap-y-1">
            <StatusDot tone={row.tone} label={row.readLabel} />
            <span className="text-label text-muted-foreground">{row.typeLabel}</span>
            <span className="text-label text-muted-foreground">{row.priorityLabel}</span>
          </span>
          <span className={row.read ? 'break-words text-body text-foreground' : 'break-words text-body font-semibold text-foreground'}>{row.title}</span>
          <span className="break-words text-body text-muted-foreground">{row.body}</span>
          <span className="text-label tabular-nums text-muted-foreground">{row.timestamp}</span>
        </span>
        {row.navigable ? <ChevronRight className="mt-1 size-4 shrink-0 text-muted-foreground" aria-hidden="true" /> : null}
      </button>
    </li>
  );
};
