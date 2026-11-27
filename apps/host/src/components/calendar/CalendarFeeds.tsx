import { ChevronDown, ChevronLeft, ChevronRight, Copy, Plus, Trash2 } from 'lucide-react';
import { Button, ConfirmDialog, Input } from '@mungini/ui-web';
import { useCalendarFeeds } from '@/hooks/calendar/useCalendarFeeds';

type Props = { propertyId: string; timezone: string };

export default function CalendarFeeds({ propertyId, timezone }: Props) {
  const view = useCalendarFeeds(propertyId, timezone);
  return (
    <div className="border-t border-border pt-4">
      <button type="button" className="flex w-full items-center justify-between text-left text-title font-semibold transition-colors duration-200 ease-in-out" aria-expanded={view.open} onClick={view.toggle}>
        {view.labels.title}<ChevronDown className={`size-4 transition-transform duration-200 ease-in-out ${view.open ? 'rotate-180' : ''}`} />
      </button>
      <div className={`grid transition-[grid-template-rows,opacity] duration-200 ease-in-out ${view.open ? 'grid-rows-[1fr] opacity-100' : 'grid-rows-[0fr] opacity-0'}`}>
        <div className="min-h-0 overflow-hidden">
          {view.open ? <div className="space-y-3 pt-3">
            <p className="text-label text-muted-foreground">{view.labels.description}</p>
            {view.error ? <p role="alert" className="text-label text-destructive">{view.error}</p> : null}
            <Button size="sm" variant="outline" icon={<Plus />} collapseLabel aria-label={view.labels.create} title={view.labels.create} disabled={view.busy || view.loading} onClick={() => void view.create()}>{view.labels.create}</Button>
            {view.link ? <div className="space-y-2"><p className="text-label text-muted-foreground">{view.labels.once}</p><div className="flex gap-2"><Input readOnly value={view.link} aria-label={view.labels.url} onFocus={(event) => event.currentTarget.select()} /><Button size="sm" variant="outline" aria-label={view.labels.copy} title={view.labels.copy} onClick={() => void view.copy()}><Copy /></Button></div>{view.copied ? <p role="status" className="text-label text-muted-foreground">{view.labels.copied}</p> : null}</div> : null}
            {view.loading ? <p className="text-label text-muted-foreground">{view.labels.loading}</p> : null}
            {!view.loading && view.items.length === 0 ? <p className="text-label text-muted-foreground">{view.labels.empty}</p> : null}
            <ul className="divide-y divide-border">{view.items.map((item) => <li key={item.id} className="flex items-center justify-between gap-2 py-2"><span className="text-body">{view.labels.created} {item.created}</span><Button variant="ghost" size="sm" disabled={view.busy} aria-label={view.labels.revoke} title={view.labels.revoke} onClick={() => view.setTarget(item.id)}><Trash2 /></Button></li>)}</ul>
            {view.pages > 1 ? <div className="flex items-center justify-end gap-2"><Button variant="ghost" size="sm" disabled={view.page <= 1 || view.loading} aria-label={view.labels.previous} title={view.labels.previous} onClick={view.previous}><ChevronLeft /></Button><span className="text-label tabular-nums">{view.page} / {view.pages}</span><Button variant="ghost" size="sm" disabled={view.page >= view.pages || view.loading} aria-label={view.labels.next} title={view.labels.next} onClick={view.next}><ChevronRight /></Button></div> : null}
          </div> : null}
        </div>
      </div>
      <ConfirmDialog open={view.target !== null} onOpenChange={(open) => { if (!open) view.setTarget(null); }} title={view.labels.confirm} description={view.labels.consequence} confirmLabel={view.labels.revoke} cancelLabel={view.labels.cancel} loading={view.busy} destructive onConfirm={() => void view.revoke()} />
    </div>
  );
}
