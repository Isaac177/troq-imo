'use client';

import Link from 'next/link';
import { Button, Select } from '@mungini/ui-web';
import { useSaveFavorite } from '@/hooks/favorites/useSaveFavorite';

type Props = { propertyId: string; city: string };

export const SaveFavorite = ({ propertyId, city }: Props) => {
  const view = useSaveFavorite(propertyId, city);
  return <section className="flex flex-col gap-2 rounded-card border border-border bg-card p-4 transition-all duration-200 ease-in-out">
    <h2 className="text-title font-semibold">{view.labels.title}</h2>
    {view.error && <p role="alert" className="text-label text-destructive">{view.labels.error}</p>}
    {view.status === 'loading' && <p className="text-label">{view.labels.loading}</p>}
    {view.status === 'anonymous' && <Link href="/login" className="text-body text-primary hover:underline">{view.labels.signIn}</Link>}
    {view.status === 'authenticated' && view.city !== view.propertyCity && <Button size="sm" variant="outline" disabled={view.busy} onClick={() => void view.switchCity()}>{view.labels.switchCity}: {view.propertyCity}</Button>}
    {view.status === 'authenticated' && view.city === view.propertyCity && <>
      {view.loading && <p className="text-label text-muted-foreground">{view.labels.loading}</p>}
      {!view.loading && view.lists.length === 0 && <p className="text-label text-muted-foreground">{view.labels.empty}</p>}
      {view.lists.length > 0 && <div className="flex flex-col gap-2 sm:flex-row">
        <Select options={view.lists.map((list) => ({ value: list.id, label: list.name }))} value={view.selected} onValueChange={view.setSelected} ariaLabel={view.labels.choose} disabled={view.loading || view.busy} />
        <Button size="sm" disabled={view.loading || view.busy || !view.selected} onClick={() => void view.toggle()}>{view.saved ? view.labels.remove : view.labels.save}</Button>
      </div>}
      <Link href="/favorites" className="text-label text-primary hover:underline">{view.labels.manage}</Link>
    </>}
  </section>;
};
