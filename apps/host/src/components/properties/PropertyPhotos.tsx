import { ArrowDown, ArrowUp, ImagePlus, Star, Trash2 } from 'lucide-react';
import { Button, ConfirmDialog, Dialog, Input, Skeleton, Switch } from '@mungini/ui-web';
import { usePropertyMedia } from '@/hooks/properties/usePropertyMedia';

type Props = { propertyId: string; onChanged: () => void };

export default function PropertyPhotos({ propertyId, onChanged }: Props) {
  const view = usePropertyMedia(propertyId, onChanged);

  return (
    <div className="border-t border-border pt-3">
      <div className="flex items-center justify-between gap-2">
        <p className="text-body font-semibold text-foreground">{view.labels.propertyMedia}</p>
        <Button size="sm" variant="outline" icon={<ImagePlus />} collapseLabel aria-label={view.labels.addPhoto} title={view.labels.addPhoto} onClick={view.openUpload}>{view.labels.addPhoto}</Button>
      </div>
      {view.error ? <p role="alert" className="mt-2 text-label text-destructive">{view.error}</p> : null}
      {view.notice ? <p role="status" className="mt-2 text-label text-muted-foreground">{view.notice}</p> : null}
      {view.loading ? <div className="mt-2 space-y-2"><Skeleton shape="line" /><Skeleton shape="block" /></div> : null}
      {!view.loading && view.media.length === 0 ? <p className="mt-2 text-label text-muted-foreground">{view.labels.noPhotos}</p> : null}
      <ul className="mt-2 grid grid-cols-1 gap-3 sm:grid-cols-2">
        {view.media.map((photo, index) => (
          <li key={photo.id} className="flex flex-col gap-2 rounded-md bg-muted/30 p-2">
            <div className="relative">
              {photo.url ? <img src={photo.url} alt={view.altFor(photo)} className="h-32 w-full rounded-md object-cover" /> : <div className="flex h-32 w-full items-center justify-center rounded-md bg-muted text-label text-muted-foreground">{photo.fileName}</div>}
              {photo.isCover ? <span className="absolute left-2 top-2 rounded-full bg-background/90 px-2 py-0.5 text-micro font-medium text-foreground">{view.labels.cover}</span> : null}
            </div>
            <p className="truncate text-label text-muted-foreground">{view.altFor(photo)}</p>
            {photo.status !== 'READY' ? <p className="text-label text-muted-foreground">{view.statusLabel(photo.status)}</p> : null}
            <div className="flex flex-wrap gap-1">
              {!photo.isCover ? <Button size="sm" variant="outline" icon={<Star />} aria-label={view.labels.setCover} title={view.labels.setCover} onClick={() => void view.setCoverPhoto(photo)}>{view.labels.setCover}</Button> : null}
              <Button size="sm" variant="ghost" disabled={index === 0} aria-label={view.labels.moveUp} title={view.labels.moveUp} onClick={() => void view.move(photo, -1)}><ArrowUp /></Button>
              <Button size="sm" variant="ghost" disabled={index === view.media.length - 1} aria-label={view.labels.moveDown} title={view.labels.moveDown} onClick={() => void view.move(photo, 1)}><ArrowDown /></Button>
              <Button size="sm" variant="outline" icon={<Trash2 />} aria-label={view.labels.removePhoto} title={view.labels.removePhoto} onClick={() => view.setRemoveTarget(photo)}>{view.labels.removePhoto}</Button>
            </div>
          </li>
        ))}
      </ul>

      <Dialog
        open={view.open}
        onOpenChange={view.setOpen}
        title={view.labels.addPhoto}
        footer={
          <div className="flex justify-end gap-2">
            <Button variant="outline" disabled={view.busy} onClick={() => view.setOpen(false)}>{view.labels.dismiss}</Button>
            <Button variant="primary" loading={view.busy} disabled={!view.picked} onClick={() => void view.submit()}>{view.labels.addPhoto}</Button>
          </div>
        }
      >
        <div className="flex flex-col gap-3">
          {view.formError ? <p role="alert" className="text-label text-destructive">{view.formError}</p> : null}
          <Button asChild variant="outline" size="sm">
            <label>
              <ImagePlus aria-hidden="true" />
              {view.labels.choosePhoto}
              <input type="file" accept="image/jpeg,image/png,image/webp" className="sr-only" onChange={(event) => { const file = event.target.files?.[0]; if (file) view.pickFile(file); }} />
            </label>
          </Button>
          {view.picked ? <p className="text-label text-muted-foreground">{view.picked.fileName}</p> : null}
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.photoAltEn}
            <Input value={view.altEn} onChange={(event) => view.setAltEn(event.target.value)} aria-label={view.labels.photoAltEn} />
          </label>
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.photoAltFr}
            <Input value={view.altFr} onChange={(event) => view.setAltFr(event.target.value)} aria-label={view.labels.photoAltFr} />
          </label>
          <Switch checked={view.cover} onCheckedChange={view.setCover} label={view.labels.setCover} />
        </div>
      </Dialog>

      <ConfirmDialog
        open={view.removeTarget !== null}
        onOpenChange={(open) => (open ? undefined : view.setRemoveTarget(null))}
        title={view.labels.confirmRemoveTitle}
        description={view.labels.confirmRemoveBody}
        confirmLabel={view.labels.removePhoto}
        cancelLabel={view.labels.dismiss}
        destructive
        loading={view.busy}
        onConfirm={() => void view.confirmRemove()}
      />
    </div>
  );
}
