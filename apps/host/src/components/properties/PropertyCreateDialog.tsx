import { Button, Dialog, Input, Textarea } from '@mungini/ui-web';
import type { PropertiesView } from '@/hooks/properties/useProperties';

type Props = { view: PropertiesView };

export default function PropertyCreateDialog({ view }: Props) {
  return (
    <Dialog
      open={view.createOpen}
      onOpenChange={view.setCreateOpen}
      title={view.labels.createProperty}
      footer={
        <div className="flex justify-end gap-2">
          <Button variant="outline" disabled={view.busy} onClick={() => view.setCreateOpen(false)}>{view.labels.close}</Button>
          <Button variant="primary" loading={view.busy} disabled={!view.createReady} onClick={() => void view.submitCreate()}>{view.labels.create}</Button>
        </div>
      }
    >
      <div className="flex flex-col gap-3">
        {view.formError ? <p role="alert" className="text-label text-destructive">{view.formError}</p> : null}
        <label className="flex flex-col gap-1 text-label text-muted-foreground">
          {view.labels.nameEn}
          <Input value={view.nameEn} onChange={(event) => view.setNameEn(event.target.value)} aria-label={view.labels.nameEn} />
        </label>
        <label className="flex flex-col gap-1 text-label text-muted-foreground">
          {view.labels.nameFr}
          <Input value={view.nameFr} onChange={(event) => view.setNameFr(event.target.value)} aria-label={view.labels.nameFr} />
        </label>
        <label className="flex flex-col gap-1 text-label text-muted-foreground">
          {view.labels.address}
          <Input value={view.addressLine} onChange={(event) => view.setAddressLine(event.target.value)} aria-label={view.labels.address} />
        </label>
        <label className="flex flex-col gap-1 text-label text-muted-foreground">
          {view.labels.descriptionEn}
          <Textarea rows={3} value={view.descriptionEn} onChange={(event) => view.setDescriptionEn(event.target.value)} aria-label={view.labels.descriptionEn} />
        </label>
        <label className="flex flex-col gap-1 text-label text-muted-foreground">
          {view.labels.descriptionFr}
          <Textarea rows={3} value={view.descriptionFr} onChange={(event) => view.setDescriptionFr(event.target.value)} aria-label={view.labels.descriptionFr} />
        </label>
        <div className="flex flex-col gap-1 text-label text-muted-foreground">
          <span>{view.labels.timezone}</span>
          <p className="text-body text-foreground">{view.timezone || '—'}</p>
        </div>
      </div>
    </Dialog>
  );
}
