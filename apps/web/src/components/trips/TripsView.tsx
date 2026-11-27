'use client';

import Link from 'next/link';
import { Button, Select, StatusDot, UnderlineTabs } from '@mungini/ui-web';
import { useTrips } from '@/hooks/trips/useTrips';

export const TripsView = () => {
  const view = useTrips();
  return (
    <main className="mx-auto flex w-full max-w-3xl flex-col gap-4 px-3 py-8 md:px-6">
      <div className="flex flex-wrap items-end justify-between gap-2">
        <h1 className="text-title font-semibold">{view.labels.title}</h1>
        {view.data ? <p className="text-label text-muted-foreground">{view.labels.tripCount}: {view.data.total}</p> : null}
      </div>
      {view.status === 'loading' && <p className="text-body">{view.labels.loading}</p>}
      {view.status === 'anonymous' && <p className="text-body">{view.labels.signInPrompt} <Link className="text-primary hover:underline" href="/login">{view.labels.signIn}</Link></p>}
      {view.status === 'authenticated' && <>
        <Select options={view.cityOptions} value={view.city} onValueChange={view.changeCity} ariaLabel={view.labels.city} />
        <UnderlineTabs items={view.tabOptions} value={view.tab || 'all'} onChange={view.changeTab} ariaLabel={view.labels.filter} />
        {view.referenceMissing ? <p role="status" className="text-label text-muted-foreground">{view.referenceMissing}</p> : null}
        {view.error && <div role="alert" className="flex items-center gap-3 text-body"><span>{view.labels.error}</span><Button size="sm" variant="outline" onClick={view.retry}>{view.labels.retry}</Button></div>}
        {view.loading && <p className="text-body text-muted-foreground">{view.labels.loading}</p>}
        {!view.loading && !view.error && view.data?.items.length === 0 && (
          <div className="flex flex-col items-start gap-2">
            <p className="text-body text-muted-foreground">{view.labels.empty}</p>
            <Link href="/" className="text-body text-primary hover:underline">{view.labels.explore}</Link>
          </div>
        )}
        {view.data && view.data.items.length > 0 && <section className="rounded-card border border-border bg-card px-4 transition-all duration-200 ease-in-out">
          {view.data.items.map((item, index) => {
            const cover = view.cover(item);
            const approval = view.approvalDeadline(item);
            return (
              <div key={item.id} className={view.isHighlighted(item) ? 'flex gap-2 border-b border-border bg-muted/40 py-4 pl-2 last:border-0' : 'flex gap-2 border-b border-border py-4 last:border-0'}>
                <span className="w-5 shrink-0 pt-0.5 text-right text-label tabular-nums text-muted-foreground">{view.rowNumber(index)}</span>
                {cover.url ? <img src={cover.url} alt={cover.alt} className="size-20 shrink-0 rounded-md object-cover" loading="lazy" /> : null}
                <div className="flex min-w-0 flex-1 flex-col gap-1">
                  <div className="flex flex-wrap items-start justify-between gap-2">
                    <Link href={`/trips/${encodeURIComponent(item.id)}`} className="min-w-0 break-words text-body font-semibold text-primary hover:underline">{view.name(item)}</Link>
                    <StatusDot tone={view.stateTone(item)} label={view.state(item)} className="shrink-0 text-label" />
                  </div>
                  {view.isHighlighted(item) ? <p className="text-label text-muted-foreground">{view.referenceMatch}</p> : null}
                  <p className="text-label tabular-nums text-muted-foreground">{view.labels.reference}: {item.reference}</p>
                  <p className="text-label text-muted-foreground">{view.unit(item)} · {view.labels.nights}: {view.nights(item)} · {view.labels.guests}: {view.guests(item)}</p>
                  <p className="text-label text-muted-foreground">{view.labels.checkIn}: {view.date(item.checkIn)} · {view.labels.checkOut}: {view.date(item.checkOut)}</p>
                  {approval ? <p className="text-label tabular-nums text-muted-foreground">{view.labels.approvalDue}: {approval}</p> : null}
                  <p className="text-body">{view.labels.total}: {view.money(item)}</p>
                </div>
              </div>
            );
          })}
        </section>}
        {view.data && view.data.total > view.data.pageSize && <div className="flex items-center justify-between gap-2"><Button size="sm" variant="outline" disabled={view.page <= 1 || view.loading} onClick={view.previous}>{view.labels.previous}</Button><span className="text-label">{view.labels.page} {view.page}</span><Button size="sm" variant="outline" disabled={view.page * view.data.pageSize >= view.data.total || view.loading} onClick={view.next}>{view.labels.next}</Button></div>}
      </>}
    </main>
  );
};
