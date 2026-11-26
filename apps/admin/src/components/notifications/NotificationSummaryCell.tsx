import { cn } from '@mungini/ui-web';

type Props = {
  title: string;
  body: string;
  typeLabel: string;
  priorityLabel: string;
  timestamp: string;
  unread: boolean;
};

export default function NotificationSummaryCell({ title, body, typeLabel, priorityLabel, timestamp, unread }: Props) {
  return (
    <div className="min-w-0">
      <p className={cn('truncate text-foreground', unread ? 'font-semibold' : 'font-medium')}>{title}</p>
      <p className="mt-0.5 line-clamp-2 text-label text-muted-foreground">{body}</p>
      <p className="mt-1 flex flex-wrap items-center gap-x-2 text-label text-muted-foreground sm:hidden">
        <span>{typeLabel}</span>
        <span aria-hidden="true">·</span>
        <span>{priorityLabel}</span>
        <span className="tabular-nums">{timestamp}</span>
      </p>
    </div>
  );
}
