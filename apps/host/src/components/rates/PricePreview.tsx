import { Calculator } from 'lucide-react';
import { Button, DatePicker, Input, Section } from '@mungini/ui-web';
import { usePricePreview } from '@/hooks/rates/usePricePreview';
import type { RatePlan } from '@/types/rates/rates';

type Props = { propertyId: string; plan: RatePlan };

export default function PricePreview({ propertyId, plan }: Props) {
  const view = usePricePreview(propertyId, plan);

  return (
    <Section title={view.labels.title} description={view.labels.hint}>
      <div className="grid grid-cols-1 gap-3 sm:grid-cols-2">
        <div className="flex flex-col gap-1 text-label text-muted-foreground">
          <span>{view.labels.checkIn}</span>
          <DatePicker value={view.checkIn} onChange={view.setCheckIn} ariaLabel={view.labels.checkIn} />
        </div>
        <div className="flex flex-col gap-1 text-label text-muted-foreground">
          <span>{view.labels.checkOut}</span>
          <DatePicker value={view.checkOut} onChange={view.setCheckOut} ariaLabel={view.labels.checkOut} />
        </div>
        <label className="flex flex-col gap-1 text-label text-muted-foreground">
          {view.labels.adults}
          <Input inputMode="numeric" value={view.adults} onChange={(event) => view.setAdults(event.target.value)} aria-label={view.labels.adults} />
        </label>
        <label className="flex flex-col gap-1 text-label text-muted-foreground">
          {view.labels.children}
          <Input inputMode="numeric" value={view.children} onChange={(event) => view.setChildren(event.target.value)} aria-label={view.labels.children} />
        </label>
      </div>
      {view.error ? <p role="alert" className="mt-3 text-label text-destructive">{view.error}</p> : null}
      <div className="mt-3 flex justify-end">
        <Button variant="primary" size="sm" icon={<Calculator />} collapseLabel aria-label={view.labels.run} title={view.labels.run} loading={view.busy} onClick={() => void view.submit()}>{view.labels.run}</Button>
      </div>
      {view.result ? (
        <div className="mt-4 border-t border-border pt-3">
          <dl className="grid grid-cols-2 gap-3 sm:grid-cols-3">
            <div><dt className="text-label text-muted-foreground">{view.labels.subtotal}</dt><dd className="text-body tabular-nums">{view.result.subtotal}</dd></div>
            <div><dt className="text-label text-muted-foreground">{view.labels.discount}</dt><dd className="text-body tabular-nums">{view.result.discount}</dd></div>
            <div><dt className="text-label text-muted-foreground">{view.labels.total}</dt><dd className="text-body font-semibold tabular-nums">{view.result.total}</dd></div>
          </dl>
          <p className="mt-4 text-body font-semibold">{view.labels.breakdown}</p>
          <div className="divide-y divide-border">
            {view.result.nights.map((night) => (
              <div key={night.date} className="py-3">
                <div className="flex justify-between gap-2 text-body"><span>{night.dateLabel}</span><span className="tabular-nums">{night.amount}</span></div>
                <ul className="mt-2 space-y-1">
                  {night.rules.map((rule) => <li key={rule.id} className="flex flex-wrap justify-between gap-1 text-label text-muted-foreground"><span>{rule.label}: {rule.amount}</span><span>{rule.outcome}</span></li>)}
                </ul>
              </div>
            ))}
          </div>
        </div>
      ) : null}
    </Section>
  );
}
