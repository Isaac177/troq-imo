import { Archive, CheckCircle2, PauseCircle } from 'lucide-react';
import { Button, ConfirmDialog, Drawer } from '@mungini/ui-web';
import type { PropertiesView } from '@/hooks/properties/useProperties';
import PropertyUnits from '@/components/properties/PropertyUnits';
import PropertyPhotos from '@/components/properties/PropertyPhotos';
import PropertyRevisions from '@/components/properties/PropertyRevisions';

type Props = { view: PropertiesView };

export default function PropertyDrawer({ view }: Props) {
  const item = view.selected;
  const canPublish = item?.status === 'APPROVED' || item?.status === 'PAUSED';
  const canPause = item?.status === 'PUBLISHED';
  const canArchive = Boolean(item && item.status !== 'ARCHIVED');
  const title = view.action === 'PUBLISH' ? view.labels.confirmPublishTitle : view.action === 'PAUSE' ? view.labels.confirmPauseTitle : view.labels.confirmArchiveTitle;
  const body = view.action === 'PUBLISH' ? view.labels.confirmPublishBody : view.action === 'PAUSE' ? view.labels.confirmPauseBody : view.labels.confirmArchiveBody;
  const confirm = view.action === 'PUBLISH' ? view.labels.publish : view.action === 'PAUSE' ? view.labels.pause : view.labels.archive;

  return (
    <>
      <Drawer
        open={item !== null}
        onOpenChange={(open) => (open ? undefined : view.setSelected(null))}
        title={view.labels.propertyDetails}
        description={item ? view.nameFor(item) : ''}
        footer={<Button variant="outline" onClick={() => view.setSelected(null)}>{view.labels.close}</Button>}
      >
        {item ? (
          <div className="flex flex-col gap-4">
            {view.lifecycleError ? <p role="alert" className="text-label text-destructive">{view.lifecycleError}</p> : null}
            <dl className="grid grid-cols-1 gap-3 text-body sm:grid-cols-2">
              <div><dt className="text-label text-muted-foreground">{view.labels.status}</dt><dd>{view.statusText(item.status)}</dd></div>
              <div><dt className="text-label text-muted-foreground">{view.labels.city}</dt><dd>{item.city}</dd></div>
              <div className="sm:col-span-2"><dt className="text-label text-muted-foreground">{view.labels.address}</dt><dd>{item.addressLine}</dd></div>
              <div><dt className="text-label text-muted-foreground">{view.labels.unitTypes}</dt><dd className="tabular-nums">{item.unitTypeCount}</dd></div>
              <div><dt className="text-label text-muted-foreground">{view.labels.units}</dt><dd className="tabular-nums">{item.unitCount}</dd></div>
              <div><dt className="text-label text-muted-foreground">{view.labels.photos}</dt><dd className="tabular-nums">{item.photoCount}</dd></div>
              <div><dt className="text-label text-muted-foreground">{view.labels.publishedRevision}</dt><dd className="tabular-nums">{item.publishedRevision}</dd></div>
              <div><dt className="text-label text-muted-foreground">{view.labels.openReservations}</dt><dd className="tabular-nums">{item.openReservations}</dd></div>
              <div><dt className="text-label text-muted-foreground">{view.labels.pendingRequests}</dt><dd className="tabular-nums">{item.pendingRequests}</dd></div>
            </dl>

            <div className="border-t border-border pt-3">
              <p className="text-label text-muted-foreground">{view.labels.guestContact}</p>
              <p className="mt-1 text-body">{item.effectiveGuestContactEmail}</p>
              <p className="text-body tabular-nums">{item.effectiveGuestContactPhone}</p>
            </div>

            {view.blockerLabels.length > 0 ? (
              <div className="border-t border-border pt-3">
                <p className="text-label text-muted-foreground">{view.labels.blockers}</p>
                <ul className="mt-1 list-inside list-disc text-body text-foreground">
                  {view.blockerLabels.map((label) => <li key={label}>{label}</li>)}
                </ul>
              </div>
            ) : null}

            <div className="flex flex-col gap-2 border-t border-border pt-3 sm:flex-row sm:flex-wrap">
              {canPublish ? <Button size="sm" variant="primary" icon={<CheckCircle2 />} onClick={() => view.openLifecycle('PUBLISH')}>{view.labels.publish}</Button> : null}
              {canPause ? <Button size="sm" variant="outline" icon={<PauseCircle />} onClick={() => view.openLifecycle('PAUSE')}>{view.labels.pause}</Button> : null}
              {canArchive ? <Button size="sm" variant="outline" icon={<Archive />} onClick={() => view.openLifecycle('ARCHIVE')}>{view.labels.archive}</Button> : null}
            </div>

            <PropertyUnits propertyId={item.id} onChanged={() => void view.refresh()} />
            <PropertyPhotos propertyId={item.id} onChanged={() => void view.refresh()} />
            <PropertyRevisions propertyId={item.id} onChanged={() => void view.refresh()} />
          </div>
        ) : null}
      </Drawer>

      <ConfirmDialog
        open={view.action !== null}
        onOpenChange={(open) => (open ? undefined : view.setAction(null))}
        title={title}
        description={body}
        confirmLabel={confirm}
        cancelLabel={view.labels.close}
        destructive={view.action === 'ARCHIVE'}
        loading={view.lifecycleBusy}
        onConfirm={() => void view.submitLifecycle()}
      />
    </>
  );
}
