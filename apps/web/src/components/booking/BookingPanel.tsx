'use client';

import Link from 'next/link';
import { Button, Checkbox, DateRangePicker, Input, Select, Textarea } from '@mungini/ui-web';
import { useBooking } from '@/hooks/booking/useBooking';
import type { StayOption } from '@/types/booking/booking';

type Props = { propertyId: string; city: string; stays: StayOption[] };

export const BookingPanel = ({ propertyId, city, stays }: Props) => {
  const view = useBooking(propertyId, city, stays);
  return <section className="flex flex-col gap-3 rounded-card border border-border bg-card p-4 transition-all duration-200 ease-in-out">
    <h2 className="text-title font-semibold">{view.t('title')}</h2>
    {view.error && <p role="alert" className="text-label text-destructive">{view.error}</p>}
    {view.status === 'loading' && <p className="text-label">{view.t('loading')}</p>}
    {view.status === 'anonymous' && <Link href="/login" className="text-body text-primary hover:underline">{view.t('signIn')}</Link>}
    {view.status === 'authenticated' && view.homeCity !== view.propertyCity && <p className="text-label text-muted-foreground">{view.t('cityMismatch')}</p>}
    {view.status === 'authenticated' && view.homeCity === view.propertyCity && view.city !== view.propertyCity && <Button size="sm" variant="outline" disabled={view.busy} onClick={() => void view.switchCity()}>{view.t('switchCity')}</Button>}
    {view.status === 'authenticated' && view.homeCity === view.propertyCity && view.city === view.propertyCity && !view.success && <>
      <div className="grid grid-cols-1 gap-3 sm:grid-cols-3">
        <label className="flex flex-col gap-1 text-label">{view.t('dates')}<DateRangePicker value={view.range} onChange={view.updateRange} disabled={view.busy} ariaLabel={view.t('dates')} /></label>
        <label className="flex flex-col gap-1 text-label">{view.t('adults')}<Input type="number" min={1} max={100} disabled={view.busy} value={view.adults} onChange={(event) => view.updateAdults(event.target.value)} /></label>
        <label className="flex flex-col gap-1 text-label">{view.t('children')}<Input type="number" min={0} max={100} disabled={view.busy} value={view.children} onChange={(event) => view.updateChildren(event.target.value)} /></label>
      </div>
      <Button size="sm" disabled={view.busy} onClick={() => void view.availability()}>{view.t('check')}</Button>
      {view.options && view.options.length === 0 && <p className="text-label">{view.t('noAvailability')}</p>}
      {view.options && view.options.length > 0 && <div className="flex flex-col gap-2 sm:flex-row">
        <Select options={view.optionsForSelect} value={view.selected} onValueChange={view.setSelected} disabled={view.busy} ariaLabel={view.t('selectOption')} placeholder={view.t('selectOption')} />
        <Button size="sm" disabled={!view.selected || view.busy} onClick={() => void view.loadQuote()}>{view.t('quote')}</Button>
      </div>}
      {view.quote && <div className="flex flex-col gap-3 border-t border-border pt-3">
        <p className="text-body font-semibold">{view.t('total')}: {view.money(view.quote.total)}</p>
        {view.expired && <p role="alert" className="text-label text-destructive">{view.t('expired')}</p>}
        {!view.policies && <p role="alert" className="text-label text-destructive">{view.t('unsupportedPolicy')}</p>}
        {view.policies && <>
          <h3 className="text-body font-semibold">{view.t('policies')}</h3>
          <p className="text-label">{view.t('cancel')}: {view.t('freeHours', { hours: Number(view.cancellation?.content.freeUntilHours) })} · {view.t('penalty', { percent: Number(view.cancellation?.content.penaltyPercent) })}</p>
          <p className="text-label">{view.t('payment')}: {view.payment?.content.collection === 'PAY_AT_PROPERTY' ? view.t('payAtProperty') : view.t('prepay')}</p>
          {!view.paymentSupported && <p role="alert" className="text-label text-destructive">{view.t('paymentUnavailable')}</p>}
          <label className="flex flex-col gap-1 text-label">{view.t('guestName')}<Input value={view.name} onChange={(event) => view.setName(event.target.value)} /></label>
          <label className="flex flex-col gap-1 text-label">{view.t('email')}<Input type="email" value={view.email} onChange={(event) => view.setEmail(event.target.value)} /></label>
          <label className="flex flex-col gap-1 text-label">{view.t('specialRequest')}<Textarea maxLength={500} value={view.request} onChange={(event) => view.setRequest(event.target.value)} /></label>
          <Checkbox checked={view.accepted} onCheckedChange={view.setAccepted} label={view.t('accept')} />
          <Button size="sm" disabled={!view.accepted || !view.paymentSupported || view.expired || view.busy} onClick={() => void view.book()}>{view.t('book')}</Button>
        </>}
      </div>}
    </>}
    {view.success && <div className="flex flex-col gap-2"><p role="status" className="text-body">{view.t('success')}</p><Link href="/trips" className="text-body text-primary hover:underline">{view.t('trips')}</Link></div>}
  </section>;
};
