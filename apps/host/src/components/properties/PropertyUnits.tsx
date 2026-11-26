import { Plus } from 'lucide-react';
import { Button, ConfirmDialog, Dialog, Input, Select, Skeleton } from '@mungini/ui-web';
import { usePropertyUnits } from '@/hooks/properties/usePropertyUnits';

type Props = { propertyId: string; onChanged: () => void };

export default function PropertyUnits({ propertyId, onChanged }: Props) {
  const view = usePropertyUnits(propertyId, onChanged);

  return (
    <div className="border-t border-border pt-3">
      <div className="flex items-center justify-between gap-2">
        <p className="text-body font-semibold text-foreground">{view.labels.unitsTitle}</p>
        <Button size="sm" variant="outline" icon={<Plus />} collapseLabel aria-label={view.labels.addUnit} title={view.labels.addUnit} onClick={view.openCreate}>{view.labels.addUnit}</Button>
      </div>
      {view.error ? <p role="alert" className="mt-2 text-label text-destructive">{view.error}</p> : null}
      {view.notice ? <p role="status" className="mt-2 text-label text-muted-foreground">{view.notice}</p> : null}
      {view.loading ? <div className="mt-2 space-y-2"><Skeleton shape="line" /><Skeleton shape="line" /></div> : null}
      {!view.loading && view.units.length === 0 ? <p className="mt-2 text-label text-muted-foreground">{view.labels.unitNoUnits}</p> : null}
      <ul className="mt-2 divide-y divide-border">
        {view.units.map((unit) => (
          <li key={unit.id} className="flex flex-wrap items-center justify-between gap-2 py-2 text-body">
            <div className="min-w-0">
              <p className="truncate font-medium text-foreground">{view.unitName(unit)}</p>
              <p className="truncate text-label text-muted-foreground">{view.categoryName(unit)} · {view.labels.unitQuantity}: {unit.sellableQuantity} · {unit.maxAdults + unit.maxChildren}</p>
            </div>
            <div className="flex gap-1">
              <Button size="sm" variant="ghost" onClick={() => view.openEdit(unit)}>{view.labels.editUnit}</Button>
              <Button size="sm" variant="outline" onClick={() => view.setArchiveTarget(unit)}>{view.labels.unitArchive}</Button>
            </div>
          </li>
        ))}
      </ul>

      <Dialog
        open={view.open}
        onOpenChange={view.setOpen}
        title={view.target ? view.labels.editUnit : view.labels.addUnit}
        footer={
          <div className="flex justify-end gap-2">
            <Button variant="outline" disabled={view.busy} onClick={() => view.setOpen(false)}>{view.labels.cancel}</Button>
            <Button variant="primary" loading={view.busy} disabled={!view.nameEn.trim() || !view.nameFr.trim() || !view.categoryCode} onClick={() => void view.submit()}>{view.labels.unitSave}</Button>
          </div>
        }
      >
        <div className="flex flex-col gap-3">
          {view.formError ? <p role="alert" className="text-label text-destructive">{view.formError}</p> : null}
          <div className="grid grid-cols-1 gap-3 sm:grid-cols-2">
            <label className="flex flex-col gap-1 text-label text-muted-foreground">
              {view.labels.unitNameEn}
              <Input value={view.nameEn} onChange={(event) => view.setNameEn(event.target.value)} aria-label={view.labels.unitNameEn} />
            </label>
            <label className="flex flex-col gap-1 text-label text-muted-foreground">
              {view.labels.unitNameFr}
              <Input value={view.nameFr} onChange={(event) => view.setNameFr(event.target.value)} aria-label={view.labels.unitNameFr} />
            </label>
            <label className="flex flex-col gap-1 text-label text-muted-foreground">
              {view.labels.unitCategory}
              <Select options={view.categoryOptions} value={view.categoryCode} onValueChange={view.setCategoryCode} ariaLabel={view.labels.unitCategory} />
            </label>
            <label className="flex flex-col gap-1 text-label text-muted-foreground">
              {view.labels.unitModel}
              <Select options={view.modelOptions} value={view.inventoryModel} onValueChange={view.setInventoryModel} ariaLabel={view.labels.unitModel} />
            </label>
            <label className="flex flex-col gap-1 text-label text-muted-foreground">
              {view.labels.unitMaxAdults}
              <Input inputMode="numeric" value={view.maxAdults} onChange={(event) => view.setMaxAdults(event.target.value)} aria-label={view.labels.unitMaxAdults} />
            </label>
            <label className="flex flex-col gap-1 text-label text-muted-foreground">
              {view.labels.unitMaxChildren}
              <Input inputMode="numeric" value={view.maxChildren} onChange={(event) => view.setMaxChildren(event.target.value)} aria-label={view.labels.unitMaxChildren} />
            </label>
            <label className="flex flex-col gap-1 text-label text-muted-foreground">
              {view.labels.unitBedrooms}
              <Input inputMode="numeric" value={view.bedrooms} onChange={(event) => view.setBedrooms(event.target.value)} aria-label={view.labels.unitBedrooms} />
            </label>
            <label className="flex flex-col gap-1 text-label text-muted-foreground">
              {view.labels.unitBathrooms}
              <Input inputMode="numeric" value={view.bathrooms} onChange={(event) => view.setBathrooms(event.target.value)} aria-label={view.labels.unitBathrooms} />
            </label>
            <label className="flex flex-col gap-1 text-label text-muted-foreground">
              {view.labels.unitBeds}
              <Input inputMode="numeric" value={view.beds} onChange={(event) => view.setBeds(event.target.value)} aria-label={view.labels.unitBeds} />
            </label>
            <label className="flex flex-col gap-1 text-label text-muted-foreground">
              {view.labels.unitQuantity}
              <Input inputMode="numeric" value={view.quantity} onChange={(event) => view.setQuantity(event.target.value)} aria-label={view.labels.unitQuantity} />
            </label>
            <label className="flex flex-col gap-1 text-label text-muted-foreground">
              {view.labels.unitDescriptionEn}
              <Input value={view.descriptionEn} onChange={(event) => view.setDescriptionEn(event.target.value)} aria-label={view.labels.unitDescriptionEn} />
            </label>
            <label className="flex flex-col gap-1 text-label text-muted-foreground">
              {view.labels.unitDescriptionFr}
              <Input value={view.descriptionFr} onChange={(event) => view.setDescriptionFr(event.target.value)} aria-label={view.labels.unitDescriptionFr} />
            </label>
          </div>
        </div>
      </Dialog>

      <ConfirmDialog
        open={view.archiveTarget !== null}
        onOpenChange={(open) => (open ? undefined : view.setArchiveTarget(null))}
        title={view.labels.confirmArchiveTitle}
        description={view.labels.confirmArchiveBody}
        confirmLabel={view.labels.unitArchive}
        cancelLabel={view.labels.cancel}
        destructive
        loading={view.busy}
        onConfirm={() => void view.confirmArchive()}
      />
    </div>
  );
}
