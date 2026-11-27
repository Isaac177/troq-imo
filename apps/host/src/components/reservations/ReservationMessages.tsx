import { Button, Skeleton, Textarea } from '@mungini/ui-web';
import { ChevronLeft, ChevronRight } from 'lucide-react';
import { useReservationMessages } from '@/hooks/reservations/useReservationMessages';

type Props = { reservationId: string; enabled: boolean; onRead?: () => void };

export default function ReservationMessages({ reservationId, enabled, onRead }: Props) {
  const view = useReservationMessages(reservationId, enabled, onRead);
  if (!enabled) return null;
  return (
    <div className="border-t border-border pt-3">
      <p className="text-body font-semibold text-foreground">{view.labels.messages}</p>
      {view.loading ? <div className="mt-2 space-y-2"><Skeleton shape="line" /><Skeleton shape="line" /></div> : null}
      {view.error ? <p role="alert" className="mt-2 text-label text-destructive">{view.error}</p> : null}
      {!view.loading && view.messages.length === 0 ? <p className="mt-2 text-label text-muted-foreground">{view.labels.empty}</p> : null}
      <ul className="mt-2 flex flex-col gap-2">
        {view.messages.map((message) => (
          <li key={message.id} className={message.senderRole === 'OPERATOR' ? 'flex justify-end' : 'flex justify-start'}>
            <div className="max-w-[85%] rounded-md bg-muted/50 px-3 py-2 text-body">
              <p className="whitespace-pre-wrap break-words text-foreground">{message.body}</p>
              <p className="mt-1 text-micro tabular-nums text-muted-foreground">{view.time(message.createdAt)}</p>
            </div>
          </li>
        ))}
      </ul>
      {view.canPrevious || view.canNext ? <div className="flex items-center justify-end gap-1"><Button variant="ghost" size="icon" onClick={view.previous} disabled={!view.canPrevious} aria-label={view.labels.previous} title={view.labels.previous}><ChevronLeft /></Button><span className="text-label tabular-nums">{view.page}</span><Button variant="ghost" size="icon" onClick={view.next} disabled={!view.canNext} aria-label={view.labels.next} title={view.labels.next}><ChevronRight /></Button></div> : null}
      <div className="mt-3 flex flex-col gap-2">
        <Textarea rows={2} maxLength={2000} value={view.body} placeholder={view.labels.placeholder} onChange={(event) => view.setBody(event.target.value)} aria-label={view.labels.placeholder} />
        <div className="flex justify-end">
          <Button size="sm" variant="primary" loading={view.sending} disabled={!view.body.trim()} onClick={() => void view.submit()}>{view.labels.send}</Button>
        </div>
      </div>
    </div>
  );
}
