'use client';

import Link from 'next/link';
import { Button } from '@mungini/ui-web';
import { useTripDetail } from '@/hooks/trips/useTripDetail';

type Props = { reservationId: string };

export const TripDetailView = ({ reservationId }: Props) => {
  const view = useTripDetail(reservationId);
  return (
    <main className="mx-auto flex w-full max-w-3xl flex-col gap-4 px-3 py-8 md:px-6">
      <Link href="/trips" className="text-body text-primary hover:underline">{view.labels.back}</Link>
      <h1 className="text-title font-semibold">{view.labels.title}</h1>
      {view.status === 'anonymous' && <p className="text-body">{view.labels.signInPrompt} <Link href="/login" className="text-primary hover:underline">{view.labels.signIn}</Link></p>}
      {(view.status === 'loading' || view.loading) && <p className="text-body">{view.labels.loading}</p>}
      {view.error && <div role="alert" className="flex gap-3 text-body"><span>{view.labels.error}</span><Button size="sm" variant="outline" onClick={view.retry}>{view.labels.retry}</Button></div>}
      {view.trip && <dl className="grid grid-cols-1 gap-4 rounded-card border border-border bg-card p-4 text-body sm:grid-cols-2">
        <div><dt className="text-muted-foreground">{view.labels.reference}</dt><dd>{view.trip.reference}</dd></div>
        <div><dt className="text-muted-foreground">{view.labels.state}</dt><dd>{view.state}</dd></div>
        <div><dt className="text-muted-foreground">{view.labels.checkIn}</dt><dd>{view.checkIn}</dd></div>
        <div><dt className="text-muted-foreground">{view.labels.checkOut}</dt><dd>{view.checkOut}</dd></div>
        <div><dt className="text-muted-foreground">{view.labels.guest}</dt><dd>{view.trip.guest.fullName}</dd></div>
        <div><dt className="text-muted-foreground">{view.labels.adults} / {view.labels.children}</dt><dd>{view.trip.adultCount} / {view.trip.childCount}</dd></div>
        <div><dt className="text-muted-foreground">{view.labels.total}</dt><dd>{view.total}</dd></div>
        {view.trip.specialRequest && <div><dt className="text-muted-foreground">{view.labels.request}</dt><dd>{view.trip.specialRequest}</dd></div>}
      </dl>}
    </main>
  );
};
