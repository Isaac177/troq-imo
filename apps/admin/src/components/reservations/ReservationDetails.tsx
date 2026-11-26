import { Button, Drawer } from '@mungini/ui-web';
import { useReservationDetails } from '@/hooks/reservations/useReservationDetails';

type Props = { id: string | null; onClose: () => void };

export default function ReservationDetails({ id, onClose }: Props) {
  const view = useReservationDetails(id);
  return (
    <Drawer open={id !== null} onOpenChange={open => { if (!open) onClose(); }} title={view.labels.title} description={view.detail?.reference}>
      {view.loading ? <p className="text-body text-muted-foreground">{view.labels.loading}</p> : null}
      {view.error ? <p className="text-body text-destructive">{view.labels.error}</p> : null}
      {view.detail ? <div className="space-y-5">
        <dl className="grid grid-cols-2 gap-3 text-body">
          <div><dt className="text-label text-muted-foreground">{view.labels.property}</dt><dd>{view.french ? view.detail.propertyNameFr || view.detail.propertyNameEn : view.detail.propertyNameEn || view.detail.propertyNameFr}</dd></div>
          <div><dt className="text-label text-muted-foreground">{view.labels.host}</dt><dd>{view.detail.operatorName}</dd></div>
          <div><dt className="text-label text-muted-foreground">{view.labels.status}</dt><dd>{view.state(view.detail.state)}</dd></div>
          <div><dt className="text-label text-muted-foreground">{view.labels.amount}</dt><dd>{view.money(view.detail.totalAmount)}</dd></div>
          <div><dt className="text-label text-muted-foreground">{view.labels.commission}</dt><dd>{view.money(view.detail.commissionAmount)}</dd></div>
          <div><dt className="text-label text-muted-foreground">{view.labels.net}</dt><dd>{view.money(view.detail.operatorNetAmount)}</dd></div>
        </dl>
        <section className="border-t border-border pt-3"><h3 className="text-body font-semibold">{view.labels.stays} ({view.detail.items.total})</h3>{view.detail.items.items.length ? <ul className="divide-y divide-border">{view.detail.items.items.map(item => <li key={item.id} className="space-y-1 py-3 text-body"><p>{view.french ? item.unitTypeNameFr || item.unitTypeNameEn : item.unitTypeNameEn || item.unitTypeNameFr}</p><p className="text-muted-foreground">{view.day(item.checkIn)} – {view.day(item.checkOut)}</p><p>{view.labels.quantity}: {item.quantity} · {view.labels.adults}: {item.adultCount} · {view.labels.children}: {item.childCount}</p><p>{view.money(item.subtotalAmount)}</p></li>)}</ul> : <p className="text-body text-muted-foreground">{view.labels.empty}</p>}</section>
        <section className="border-t border-border pt-3"><h3 className="text-body font-semibold">{view.labels.guests} ({view.detail.guests.total})</h3>{view.detail.guests.items.length ? <ul className="divide-y divide-border">{view.detail.guests.items.map((guest, index) => <li key={`${guest.itemId}-${index}`} className="py-2 text-body"><p>{guest.fullName}</p><p className="text-muted-foreground">{guest.contactEmail || guest.contactPhone}</p></li>)}</ul> : <p className="text-body text-muted-foreground">{view.labels.empty}</p>}</section>
        <dl className="grid grid-cols-3 gap-2 border-t border-border pt-3 text-body"><div><dt className="text-muted-foreground">{view.labels.cases}</dt><dd>{view.detail.cases.total}</dd></div><div><dt className="text-muted-foreground">{view.labels.amendments}</dt><dd>{view.detail.amendments.total}</dd></div><div><dt className="text-muted-foreground">{view.labels.timeline}</dt><dd>{view.detail.timeline.total}</dd></div></dl>
        {view.pageCount > 1 ? <div className="flex items-center justify-between gap-2"><Button variant="ghost" size="sm" disabled={view.page <= 1 || view.loading} onClick={() => view.setPage(view.page - 1)}>{view.labels.previous}</Button><span className="text-label">{view.labels.page} {view.page} / {view.pageCount}</span><Button variant="ghost" size="sm" disabled={view.page >= view.pageCount || view.loading} onClick={() => view.setPage(view.page + 1)}>{view.labels.next}</Button></div> : null}
      </div> : null}
    </Drawer>
  );
}
