'use client';

import Link from 'next/link';
import { Button, Select } from '@mungini/ui-web';
import { useTrips } from '@/hooks/trips/useTrips';

export const TripsView = () => {
  const view = useTrips();
  return (
    <main className="mx-auto flex w-full max-w-3xl flex-col gap-4 px-3 py-8 md:px-6">
      <h1 className="text-title font-semibold">{view.labels.title}</h1>
      {view.status === 'loading' && <p className="text-body">{view.labels.loading}</p>}
      {view.status === 'anonymous' && <p className="text-body">{view.labels.signInPrompt} <Link className="text-primary hover:underline" href="/login">{view.labels.signIn}</Link></p>}
      {view.status === 'authenticated' && <>
        <div className="grid grid-cols-1 gap-3 sm:grid-cols-2">
          <Select options={view.cityOptions} value={view.city} onValueChange={view.changeCity} ariaLabel={view.labels.city} />
          <Select options={view.tabOptions} value={view.tab || 'all'} onValueChange={view.changeTab} ariaLabel={view.labels.filter} />
        </div>
        {view.error && <div role="alert" className="flex items-center gap-3 text-body"><span>{view.labels.error}</span><Button size="sm" variant="outline" onClick={view.retry}>{view.labels.retry}</Button></div>}
        {view.loading && <p className="text-body text-muted-foreground">{view.labels.loading}</p>}
        {!view.loading && !view.error && view.data?.items.length === 0 && <p className="text-body text-muted-foreground">{view.labels.empty}</p>}
        {view.data && view.data.items.length > 0 && <section className="rounded-card border border-border bg-card px-4 transition-all duration-200 ease-in-out">
          {view.data.items.map((item) => <div key={item.id} className="flex flex-col gap-2 border-b border-border py-4 last:border-0">
            <div className="flex flex-wrap items-start justify-between gap-2"><Link href={`/trips/${encodeURIComponent(item.id)}`} className="min-w-0 break-words text-body font-semibold text-primary hover:underline">{view.name(item)}</Link><span className="text-label text-muted-foreground">{view.state(item)}</span></div>
            <p className="text-label text-muted-foreground">{view.unit(item)}</p>
            <p className="text-label text-muted-foreground">{view.labels.checkIn}: {view.date(item.checkIn)} · {view.labels.checkOut}: {view.date(item.checkOut)}</p>
            <p className="text-body">{view.labels.total}: {view.money(item)}</p>
          </div>)}
        </section>}
        {view.data && view.data.total > view.data.pageSize && <div className="flex items-center justify-between gap-2"><Button size="sm" variant="outline" disabled={view.page <= 1 || view.loading} onClick={view.previous}>{view.labels.previous}</Button><span className="text-label">{view.labels.page} {view.page}</span><Button size="sm" variant="outline" disabled={view.page * view.data.pageSize >= view.data.total || view.loading} onClick={view.next}>{view.labels.next}</Button></div>}
      </>}
    </main>
  );
};
