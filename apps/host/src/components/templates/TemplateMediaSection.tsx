import { ArrowDown, ArrowUp, ImagePlus, Star, Trash2 } from 'lucide-react';
import { Button, Input, Skeleton, Switch } from '@mungini/ui-web';
import { useTemplateMedia } from '@/hooks/templates/useTemplateMedia';

type Props = { templateId: string; onChanged: () => void };

export default function TemplateMediaSection({ templateId, onChanged }: Props) {
  const view = useTemplateMedia(templateId, onChanged);

  return (
    <div className="border-t border-border pt-3">
      <div className="flex items-center justify-between gap-2">
        <p className="text-body font-semibold text-foreground">{view.labels.title}</p>
        <Button size="sm" variant="outline" icon={<ImagePlus />} collapseLabel aria-label={view.labels.addPhoto} title={view.labels.addPhoto} aria-expanded={view.uploadOpen} onClick={view.openUpload}>{view.labels.addPhoto}</Button>
      </div>

      {view.error ? <p role="alert" className="mt-2 text-label text-destructive">{view.error}</p> : null}
      {view.notice ? <p role="status" className="mt-2 text-label text-muted-foreground">{view.notice}</p> : null}
      {view.loading ? <div className="mt-2 space-y-2"><Skeleton shape="line" /><Skeleton shape="block" /></div> : null}
      {!view.loading && view.media.length === 0 ? <p className="mt-2 text-label text-muted-foreground">{view.labels.noPhotos}</p> : null}

      <ul className="mt-2 divide-y divide-border">
        {view.media.map((photo, index) => (
          <li key={photo.id} className="flex flex-col gap-2 py-2 sm:flex-row sm:items-center sm:justify-between">
            <div className="flex min-w-0 items-center gap-2">
              {photo.url ? <img src={photo.url} alt={view.altFor(photo)} loading="lazy" className="h-14 w-20 shrink-0 rounded-lg object-cover" /> : <span className="flex h-14 w-20 shrink-0 items-center justify-center rounded-lg bg-muted px-1 text-center text-micro text-muted-foreground">{photo.fileName}</span>}
              <div className="min-w-0">
                <p className="truncate text-body text-foreground">{view.altFor(photo)}</p>
                <p className="text-label text-muted-foreground">{photo.isCover ? `${view.labels.cover} · ${view.statusLabel(photo.status)}` : view.statusLabel(photo.status)}</p>
              </div>
            </div>
            <div className="flex flex-wrap items-center gap-1">
              {!photo.isCover ? <Button size="sm" variant="outline" icon={<Star />} collapseLabel aria-label={view.labels.setCover} title={view.labels.setCover} onClick={() => void view.setCoverPhoto(photo)}>{view.labels.setCover}</Button> : null}
              <Button size="icon" variant="ghost" disabled={index === 0} aria-label={view.labels.moveUp} title={view.labels.moveUp} onClick={() => void view.move(photo, -1)}><ArrowUp /></Button>
              <Button size="icon" variant="ghost" disabled={index === view.media.length - 1} aria-label={view.labels.moveDown} title={view.labels.moveDown} onClick={() => void view.move(photo, 1)}><ArrowDown /></Button>
              <Button size="sm" variant="outline" icon={<Trash2 />} collapseLabel aria-label={view.labels.removePhoto} title={view.labels.removePhoto} onClick={() => view.setRemoveTarget(photo)}>{view.labels.removePhoto}</Button>
            </div>
          </li>
        ))}
      </ul>

      {view.uploadOpen ? (
        <div className="mt-3 flex animate-fade-in flex-col gap-3 border-t border-border pt-3">
          {view.formError ? <p role="alert" className="text-label text-destructive">{view.formError}</p> : null}
          <Button asChild variant="outline" size="sm">
            <label>
              <ImagePlus aria-hidden="true" />
              {view.labels.choosePhoto}
              <input type="file" accept="image/jpeg,image/png,image/webp" className="sr-only" onChange={view.chooseFile} />
            </label>
          </Button>
          {view.picked ? <p className="text-label text-muted-foreground">{view.picked.fileName}</p> : null}
          <div className="grid grid-cols-1 gap-3 sm:grid-cols-2">
            <label className="flex flex-col gap-1 text-label text-muted-foreground">
              {view.labels.altEn}
              <Input value={view.altEn} onChange={(event) => view.setAltEn(event.target.value)} aria-label={view.labels.altEn} />
            </label>
            <label className="flex flex-col gap-1 text-label text-muted-foreground">
              {view.labels.altFr}
              <Input value={view.altFr} onChange={(event) => view.setAltFr(event.target.value)} aria-label={view.labels.altFr} />
            </label>
          </div>
          <Switch checked={view.cover} onCheckedChange={view.setCover} label={view.labels.coverPhoto} />
          <div className="flex justify-end gap-2">
            <Button size="sm" variant="outline" disabled={view.busy} onClick={view.closeUpload}>{view.labels.cancel}</Button>
            <Button size="sm" variant="primary" loading={view.busy} disabled={!view.picked} onClick={() => void view.submit()}>{view.labels.upload}</Button>
          </div>
        </div>
      ) : null}

      {view.removeTarget ? (
        <div className="mt-3 flex animate-fade-in flex-col gap-2 border-t border-border pt-3 sm:flex-row sm:items-center sm:justify-between">
          <p className="text-label text-muted-foreground">{view.labels.removeBody}</p>
          <div className="flex justify-end gap-2">
            <Button size="sm" variant="outline" disabled={view.busy} onClick={view.cancelRemove}>{view.labels.dismiss}</Button>
            <Button size="sm" variant="destructive" loading={view.busy} onClick={() => void view.confirmRemove()}>{view.labels.removePhoto}</Button>
          </div>
        </div>
      ) : null}
    </div>
  );
}
