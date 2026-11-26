'use client';

import Link from 'next/link';
import { Button, Input, Select } from '@mungini/ui-web';
import { useFavorites } from '@/hooks/favorites/useFavorites';

export const FavoritesView = () => {
  const view = useFavorites();
  return <main className="mx-auto flex w-full max-w-3xl flex-col gap-4 px-3 py-8 md:px-6">
    <h1 className="text-title font-semibold">{view.labels.title}</h1>
    {view.status === 'loading' && <p className="text-body">{view.labels.loading}</p>}
    {view.status === 'anonymous' && <p className="text-body">{view.labels.signInSubtitle} <Link className="text-primary hover:underline" href="/login">{view.labels.signIn}</Link></p>}
    {view.status === 'authenticated' && <>
      <Select options={view.cities.map((city) => ({ value: city.code, label: city.name }))} value={view.city} onValueChange={view.changeCity} ariaLabel={view.labels.city} />
      {view.error && <p role="alert" className="text-label text-destructive">{view.labels.error}</p>}
      <div className="flex flex-col gap-2 rounded-card border border-border bg-card p-4 transition-all duration-200 ease-in-out sm:flex-row">
        <Input value={view.name} onChange={(event) => view.setName(event.target.value)} aria-label={view.labels.name} placeholder={view.labels.name} maxLength={80} />
        <Button size="sm" disabled={view.busy || !view.name.trim()} onClick={() => void view.create()}>{view.labels.create}</Button>
      </div>
      {view.loading ? <p className="text-body">{view.labels.loading}</p> : null}
      {!view.loading && !view.lists?.items.length && <p className="text-body text-muted-foreground">{view.labels.empty}</p>}
      {view.lists && view.lists.items.length > 0 && <section className="flex flex-col gap-3 rounded-card border border-border bg-card p-4 transition-all duration-200 ease-in-out">
        <Select options={view.lists.items.map((list) => ({ value: list.id, label: list.name }))} value={view.selected} onValueChange={view.select} ariaLabel={view.labels.title} />
        {view.lists.total > view.lists.pageSize && <div className="flex items-center justify-between gap-2"><Button size="sm" variant="outline" disabled={view.listPage <= 1} onClick={view.previousList}>{view.labels.previous}</Button><span className="text-label">{view.labels.page} {view.listPage}</span><Button size="sm" variant="outline" disabled={view.listPage * view.lists.pageSize >= view.lists.total} onClick={view.nextList}>{view.labels.next}</Button></div>}
        {view.loadingItems && <p className="text-label text-muted-foreground">{view.labels.loading}</p>}
        {view.items && view.items.items.length === 0 && <p className="text-label text-muted-foreground">{view.labels.noItems}</p>}
        {view.items?.items.map((item) => <div key={item.propertyId} className="flex items-center justify-between gap-3 border-t border-border py-3"><Link href={`/stays/${encodeURIComponent(item.propertyId)}?city=${encodeURIComponent(view.city)}`} className="min-w-0 break-words text-body text-primary hover:underline">{item.name}</Link><Button size="sm" variant="outline" disabled={view.busy} onClick={() => void view.remove(item.propertyId)}>{view.labels.remove}</Button></div>)}
        {view.items && view.items.total > view.items.pageSize && <div className="flex items-center justify-between gap-2"><Button size="sm" variant="outline" disabled={view.itemPage <= 1} onClick={view.previousItem}>{view.labels.previous}</Button><span className="text-label">{view.labels.page} {view.itemPage}</span><Button size="sm" variant="outline" disabled={view.itemPage * view.items.pageSize >= view.items.total} onClick={view.nextItem}>{view.labels.next}</Button></div>}
      </section>}
    </>}
  </main>;
};
