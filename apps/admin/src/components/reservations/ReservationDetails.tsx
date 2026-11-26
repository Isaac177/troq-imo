import { Button, Drawer } from '@mungini/ui-web';
import { useReservationDetails } from '@/hooks/reservations/useReservationDetails';

type Props = { id: string | null; onClose: () => void };

export default function ReservationDetails({ id, onClose }: Props) {
  const view = useReservationDetails(id);
  return (
    <Drawer open={id !== null} onOpenChange={(open) => { if (!open) onClose(); }} title={view.labels.title} description={view.detail?.reference}>
      {view.loading ? <p className="text-body text-muted-foreground">{view.labels.loading}</p> : null}
      {view.error ? <p className="text-body text-destructive">{view.labels.error}</p> : null}
      {view.detail ? (
        <div className="space-y-5">
          <dl className="grid grid-cols-1 gap-3 text-body sm:grid-cols-2">
            <div><dt className="text-label text-muted-foreground">{view.labels.property}</dt><dd>{view.french ? view.detail.propertyNameFr || view.detail.propertyNameEn : view.detail.propertyNameEn || view.detail.propertyNameFr}</dd></div>
            <div><dt className="text-label text-muted-foreground">{view.labels.host}</dt><dd>{view.detail.operatorName}</dd></div>
            <div><dt className="text-label text-muted-foreground">{view.labels.status}</dt><dd>{view.state(view.detail.state)}</dd></div>
            <div><dt className="text-label text-muted-foreground">{view.labels.amount}</dt><dd className="tabular-nums">{view.money(view.detail.totalAmount)}</dd></div>
            <div><dt className="text-label text-muted-foreground">{view.labels.commission}</dt><dd className="tabular-nums">{view.money(view.detail.commissionAmount)}</dd></div>
            <div><dt className="text-label text-muted-foreground">{view.labels.net}</dt><dd className="tabular-nums">{view.money(view.detail.operatorNetAmount)}</dd></div>
          </dl>

          <section className="border-t border-border pt-3">
            <h3 className="text-body font-semibold">{view.labels.stays} ({view.detail.items.total})</h3>
            {view.detail.items.items.length ? (
              <ul className="divide-y divide-border">
                {view.detail.items.items.map((item) => (
                  <li key={item.id} className="space-y-1 py-3 text-body">
                    <p>{view.french ? item.unitTypeNameFr || item.unitTypeNameEn : item.unitTypeNameEn || item.unitTypeNameFr}</p>
                    <p className="tabular-nums text-muted-foreground">{view.day(item.checkIn)} – {view.day(item.checkOut)}</p>
                    <p>{view.labels.quantity}: {item.quantity} · {view.labels.adults}: {item.adultCount} · {view.labels.children}: {item.childCount}</p>
                    <p className="tabular-nums">{view.money(item.subtotalAmount)}</p>
                  </li>
                ))}
              </ul>
            ) : <p className="text-body text-muted-foreground">{view.labels.empty}</p>}
          </section>

          <section className="border-t border-border pt-3">
            <h3 className="text-body font-semibold">{view.labels.guests} ({view.detail.guests.total})</h3>
            {view.detail.guests.items.length ? (
              <ul className="divide-y divide-border">
                {view.detail.guests.items.map((guest, index) => (
                  <li key={`${guest.itemId}-${index}`} className="py-2 text-body">
                    <p>{guest.fullName}</p>
                    <p className="text-label text-muted-foreground">{guest.contactEmail || guest.contactPhone}</p>
                  </li>
                ))}
              </ul>
            ) : <p className="text-body text-muted-foreground">{view.labels.empty}</p>}
          </section>

          <section className="border-t border-border pt-3">
            <h3 className="text-body font-semibold">{view.labels.cases} ({view.detail.cases.total})</h3>
            {view.detail.cases.items.length ? (
              <ul className="divide-y divide-border">
                {view.detail.cases.items.map((item) => (
                  <li key={item.id} className="flex flex-wrap items-center justify-between gap-2 py-2 text-body">
                    <span>{view.caseType(item.caseType)}</span>
                    <span className="text-label text-muted-foreground">{view.caseState(item.state)} · {view.labels.caseDeadline}: <span className="tabular-nums">{view.datetime(item.deadlineAt)}</span></span>
                  </li>
                ))}
              </ul>
            ) : <p className="text-body text-muted-foreground">{view.labels.noCases}</p>}
          </section>

          <section className="border-t border-border pt-3">
            <h3 className="text-body font-semibold">{view.labels.amendments} ({view.detail.amendments.total})</h3>
            {view.detail.amendments.items.length ? (
              <ul className="divide-y divide-border">
                {view.detail.amendments.items.map((item) => (
                  <li key={item.id} className="flex flex-wrap items-center justify-between gap-2 py-2 text-body">
                    <span className="tabular-nums">{view.labels.amendmentDelta}: {view.money(item.deltaAmount)}</span>
                    <span className="text-label text-muted-foreground">{view.amendmentState(item.state)} · {view.labels.amendmentExpires}: <span className="tabular-nums">{view.datetime(item.expiresAt)}</span></span>
                  </li>
                ))}
              </ul>
            ) : <p className="text-body text-muted-foreground">{view.labels.noAmendments}</p>}
          </section>

          <section className="border-t border-border pt-3">
            <h3 className="text-body font-semibold">{view.labels.timeline} ({view.detail.timeline.total})</h3>
            {view.detail.timeline.items.length ? (
              <ul className="divide-y divide-border">
                {view.detail.timeline.items.map((item) => (
                  <li key={item.id} className="space-y-0.5 py-2 text-body">
                    <p>{view.actionLabel(item.action)}</p>
                    {item.reason ? <p className="text-label text-muted-foreground">{item.reason}</p> : null}
                    <p className="text-label tabular-nums text-muted-foreground">{view.datetime(item.createdAt)}</p>
                  </li>
                ))}
              </ul>
            ) : <p className="text-body text-muted-foreground">{view.labels.noTimeline}</p>}
          </section>

          {view.pageCount > 1 ? (
            <div className="flex items-center justify-between gap-2">
              <Button variant="ghost" size="sm" disabled={view.page <= 1 || view.loading} onClick={() => view.setPage(view.page - 1)}>{view.labels.previous}</Button>
              <span className="text-label">{view.labels.page} {view.page} / {view.pageCount}</span>
              <Button variant="ghost" size="sm" disabled={view.page >= view.pageCount || view.loading} onClick={() => view.setPage(view.page + 1)}>{view.labels.next}</Button>
            </div>
          ) : null}
        </div>
      ) : null}
    </Drawer>
  );
}
