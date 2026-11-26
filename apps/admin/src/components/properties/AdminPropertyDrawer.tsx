import { ShieldAlert, Undo2 } from 'lucide-react';
import { Button, Dialog, Drawer, Select, Skeleton } from '@mungini/ui-web';
import { useAdminPropertyDetail } from '@/hooks/properties/useAdminPropertyDetail';

type Props = { propertyId: string | null; onClose: () => void; onChanged: () => void };

export default function AdminPropertyDrawer({ propertyId, onClose, onChanged }: Props) {
  const view = useAdminPropertyDetail(propertyId, onChanged);
  const item = view.detail;
  return (
    <>
      <Drawer open={propertyId !== null} onOpenChange={(open) => (open ? undefined : onClose())} title={view.labels.property} description={view.name} footer={<Button variant="outline" onClick={onClose}>{view.labels.close}</Button>}>
        {view.loading && !item ? <div className="space-y-2"><Skeleton shape="line" /><Skeleton shape="line" /><Skeleton shape="block" /></div> : null}
        {view.error ? <p role="alert" className="text-label text-destructive">{view.labels.error}</p> : null}
        {view.notice ? <p role="status" className="text-label text-muted-foreground">{view.notice}</p> : null}
        {view.formError ? <p role="alert" className="text-label text-destructive">{view.formError}</p> : null}
        {item ? (
          <div className="flex flex-col gap-4">
            <dl className="grid grid-cols-1 gap-3 text-body sm:grid-cols-2">
              <div><dt className="text-label text-muted-foreground">{view.labels.status}</dt><dd>{view.statusLabel(item.status)}</dd></div>
              <div><dt className="text-label text-muted-foreground">{view.labels.host}</dt><dd>{item.hostName} · {item.hostStatus}</dd></div>
              <div><dt className="text-label text-muted-foreground">{view.labels.city}</dt><dd>{item.city}</dd></div>
              <div><dt className="text-label text-muted-foreground">{view.labels.neighborhood}</dt><dd>{item.neighborhood}</dd></div>
              <div className="sm:col-span-2"><dt className="text-label text-muted-foreground">{view.labels.address}</dt><dd>{item.addressLine}</dd></div>
              <div><dt className="text-label text-muted-foreground">{view.labels.unitTypes} / {view.labels.units}</dt><dd className="tabular-nums">{item.unitTypeCount} / {item.unitCount}</dd></div>
              <div><dt className="text-label text-muted-foreground">{view.labels.photos}</dt><dd className="tabular-nums">{item.photoCount}</dd></div>
              <div><dt className="text-label text-muted-foreground">{view.labels.publishedRevision}</dt><dd className="tabular-nums">{item.publishedRevision}</dd></div>
              <div><dt className="text-label text-muted-foreground">{view.labels.pendingRequests}</dt><dd className="tabular-nums">{item.pendingRequests}</dd></div>
              <div><dt className="text-label text-muted-foreground">{view.labels.openReservations}</dt><dd className="tabular-nums">{item.openReservations}</dd></div>
            </dl>

            <div className="border-t border-border pt-3">
              <p className="text-label text-muted-foreground">{view.labels.descriptionEn}</p>
              <p className="mt-1 whitespace-pre-wrap text-body">{item.descriptionEn || '—'}</p>
              <p className="mt-2 text-label text-muted-foreground">{view.labels.descriptionFr}</p>
              <p className="mt-1 whitespace-pre-wrap text-body">{item.descriptionFr || '—'}</p>
            </div>

            <div className="border-t border-border pt-3">
              <dl className="grid grid-cols-1 gap-3 text-body sm:grid-cols-2">
                <div><dt className="text-label text-muted-foreground">{view.labels.checkIn} / {view.labels.checkOut}</dt><dd className="tabular-nums">{item.checkInFrom ?? '—'} / {item.checkOutBy ?? '—'}</dd></div>
                <div><dt className="text-label text-muted-foreground">{view.labels.pets} / {view.labels.smoking}</dt><dd>{item.petsPolicy ?? '—'} / {item.smokingPolicy ?? '—'}</dd></div>
                <div className="sm:col-span-2"><dt className="text-label text-muted-foreground">{view.labels.amenities}</dt><dd className="break-words">{view.amenities || '—'}</dd></div>
                <div className="sm:col-span-2"><dt className="text-label text-muted-foreground">{view.labels.guestContact}</dt><dd>{item.guestContactEmail} · {item.guestContactPhone}</dd></div>
              </dl>
            </div>

            <div className="border-t border-border pt-3">
              <p className="text-label text-muted-foreground">{view.labels.houseRulesEn}</p>
              <p className="mt-1 whitespace-pre-wrap text-body">{item.houseRulesEn || '—'}</p>
              <p className="mt-2 text-label text-muted-foreground">{view.labels.houseRulesFr}</p>
              <p className="mt-1 whitespace-pre-wrap text-body">{item.houseRulesFr || '—'}</p>
            </div>

            <div className="flex flex-col gap-2 border-t border-border pt-3 sm:flex-row">
              {view.canSuspend ? <Button size="sm" variant="destructive" icon={<ShieldAlert />} onClick={() => view.setSuspendOpen(true)}>{view.labels.suspend}</Button> : null}
              {view.canReinstate ? <Button size="sm" variant="primary" icon={<Undo2 />} loading={view.busy} onClick={() => void view.reinstate()}>{view.labels.reinstate}</Button> : null}
            </div>
          </div>
        ) : null}
      </Drawer>

      <Dialog
        open={view.suspendOpen}
        onOpenChange={view.setSuspendOpen}
        title={view.labels.suspendTitle}
        description={view.labels.suspendBody}
        footer={
          <div className="flex justify-end gap-2">
            <Button variant="outline" disabled={view.busy} onClick={() => view.setSuspendOpen(false)}>{view.labels.close}</Button>
            <Button variant="destructive" loading={view.busy} disabled={!view.reason} onClick={() => void view.submitSuspend()}>{view.labels.save}</Button>
          </div>
        }
      >
        <label className="flex flex-col gap-1 text-label text-muted-foreground">
          {view.labels.reason}
          <Select options={view.reasonOptions} value={view.reason} onValueChange={view.setReason} placeholder={view.labels.chooseReason} ariaLabel={view.labels.reason} />
        </label>
      </Dialog>
    </>
  );
}
