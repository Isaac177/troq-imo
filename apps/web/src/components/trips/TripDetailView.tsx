'use client';

import Link from 'next/link';
import { Button } from '@mungini/ui-web';
import { useTripDetail } from '@/hooks/trips/useTripDetail';
import { TripActions } from '@/components/trips/TripActions';

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
      {view.trip && view.coverUrl ? <img src={view.coverUrl} alt={view.trip.reference} className="h-[180px] w-full rounded-card object-cover" /> : null}
      {view.trip ? (
        <div className="flex flex-col gap-4">
          <div className="flex flex-col gap-1">
            <p className="text-body font-medium text-foreground">{view.state}</p>
            {view.stateInfo ? <p className="text-label text-muted-foreground">{view.stateInfo}</p> : null}
          </div>
          <dl className="grid grid-cols-1 gap-4 rounded-card border border-border bg-card p-4 text-body sm:grid-cols-2">
            <div><dt className="text-muted-foreground">{view.labels.reference}</dt><dd>{view.trip.reference}</dd></div>
            <div><dt className="text-muted-foreground">{view.labels.status}</dt><dd>{view.state}</dd></div>
            <div><dt className="text-muted-foreground">{view.labels.checkIn}</dt><dd>{view.checkIn}</dd></div>
            <div><dt className="text-muted-foreground">{view.labels.checkOut}</dt><dd>{view.checkOut}</dd></div>
            <div><dt className="text-muted-foreground">{view.labels.nights}</dt><dd className="tabular-nums">{view.nights}</dd></div>
            <div><dt className="text-muted-foreground">{view.labels.guest}</dt><dd>{view.trip.guest.fullName}</dd></div>
            <div><dt className="text-muted-foreground">{view.labels.adults} / {view.labels.children}</dt><dd>{view.trip.adultCount} / {view.trip.childCount}</dd></div>
            <div><dt className="text-muted-foreground">{view.labels.total}</dt><dd className="tabular-nums">{view.total}</dd></div>
            {view.approvalDeadline ? <div><dt className="text-muted-foreground">{view.labels.approvalDeadline}</dt><dd className="tabular-nums">{view.approvalDeadline}</dd></div> : null}
            {view.holdExpiry ? <div><dt className="text-muted-foreground">{view.labels.holdExpiry}</dt><dd className="tabular-nums">{view.holdExpiry}</dd></div> : null}
            {view.trip.specialRequest ? <div className="sm:col-span-2"><dt className="text-muted-foreground">{view.labels.request}</dt><dd>{view.trip.specialRequest}</dd></div> : null}
          </dl>

          <div className="rounded-card border border-border bg-card p-4 text-body">
            <p className="text-label text-muted-foreground">{view.labels.nightlyRate}</p>
            <ul className="mt-1 divide-y divide-border">
              {view.nightLines.map((night) => (
                <li key={night.date} className="flex justify-between gap-4 py-1.5">
                  <span className="tabular-nums text-muted-foreground">{night.date}</span>
                  <span className="tabular-nums">{night.amount}</span>
                </li>
              ))}
            </ul>
            <div className="mt-2 flex justify-between gap-4 border-t border-border pt-2 font-medium">
              <span>{view.labels.total}</span>
              <span className="tabular-nums">{view.total}</span>
            </div>
          </div>

          <div className="flex flex-col gap-2 text-body">
            <p className="text-label text-muted-foreground">{view.labels.contact}</p>
            {view.contactEmail ? <a className="text-primary hover:underline" href={`mailto:${view.contactEmail}`}>{view.contactEmail}</a> : null}
            {view.contactPhone ? <a className="text-primary hover:underline" href={`tel:${view.contactPhone}`}>{view.contactPhone}</a> : null}
            <Link className="text-primary hover:underline" href={view.propertyHref}>{view.labels.viewListing}</Link>
          </div>

          {view.actions.started && view.trip.state === 'CONFIRMED' ? <p className="text-label text-muted-foreground">{view.labels.started}</p> : null}
          <TripActions view={view.actions} />
        </div>
      ) : null}
    </main>
  );
};
