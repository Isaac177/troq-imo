import { ChevronLeft, ChevronRight, Search } from 'lucide-react';
import { Button, EmptyState, Input, Section, Select, Skeleton } from '@mungini/ui-web';
import ReservationMessages from '@/components/reservations/ReservationMessages';
import { useHostMessages } from '@/hooks/messages/useHostMessages';

export default function MessagesView() {
  const view = useHostMessages();
  return (
    <Section title={view.labels.title} description={view.labels.subtitle} actions={
      <div className="flex w-full flex-col gap-2 sm:w-auto sm:flex-row">
        {view.hosts.length > 1 ? <Select options={view.hosts} value={view.hostId} onValueChange={view.setHostId} ariaLabel={view.labels.host} /> : null}
        <Input type="search" value={view.search} onChange={(event) => view.setSearch(event.target.value)} placeholder={view.labels.search} aria-label={view.labels.search} leadingIcon={<Search />} />
      </div>
    }>
      {view.error ? <p role="alert" className="text-label text-destructive">{view.error}</p> : null}
      <div className="grid gap-4 md:grid-cols-2">
        <div className="min-w-0">
          {view.loading ? <Skeleton shape="line" /> : null}
          {!view.loading && !view.rows.length && !view.error ? <EmptyState title={view.labels.empty} /> : null}
          <ul className="divide-y divide-border">
            {view.rows.map((row) => (
              <li key={row.reservationId}>
                <button type="button" onClick={() => view.setSelected(row.reservationId)} aria-pressed={view.selected === row.reservationId} className="h-9 w-full rounded-md px-2 text-left transition-colors duration-200 ease-in-out hover:bg-muted focus-visible:outline focus-visible:outline-2 focus-visible:outline-ring aria-pressed:bg-muted">
                  <span className="flex items-center justify-between gap-2"><span className="truncate text-body font-medium">{view.name(row)}</span><span className="shrink-0 text-label tabular-nums text-muted-foreground">{view.time(row)}</span></span>
                  <span className="flex min-w-0 items-center gap-2 text-label text-muted-foreground"><span className="truncate">{row.reference} · {row.lastBody}</span>{row.unreadCount > 0 ? <span className="shrink-0 font-medium text-foreground">{view.unread(row.unreadCount)}</span> : null}</span>
                </button>
              </li>
            ))}
          </ul>
          <div className="mt-3 flex items-center justify-between"><span className="text-label tabular-nums">{view.total}</span><div className="flex items-center gap-1"><Button variant="ghost" size="icon" onClick={view.previous} disabled={!view.canPrevious} aria-label={view.labels.previous} title={view.labels.previous}><ChevronLeft /></Button><span className="text-label tabular-nums">{view.page}</span><Button variant="ghost" size="icon" onClick={view.next} disabled={!view.canNext} aria-label={view.labels.next} title={view.labels.next}><ChevronRight /></Button></div></div>
        </div>
        <div className="min-w-0">
          {view.selected ? <ReservationMessages key={view.selected} reservationId={view.selected} enabled onRead={view.onRead} /> : <EmptyState title={view.labels.choose} />}
        </div>
      </div>
    </Section>
  );
}
