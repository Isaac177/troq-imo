import { Button, Checkbox, Dialog, Input, PhoneField, Select, Textarea } from '@mungini/ui-web';
import { useRevisionEditor } from '@/hooks/properties/useRevisionEditor';
import type { Revision } from '@/services/properties/properties.api';

type Props = { propertyId: string; revision: Revision | null; onSaved: (next: Revision) => void; onClose: () => void; onSubmitted: () => void };

export default function RevisionEditorDialog({ propertyId, revision, onSaved, onClose, onSubmitted }: Props) {
  const view = useRevisionEditor(propertyId, revision, onSaved, onSubmitted);
  return (
    <Dialog
      open={revision !== null}
      onOpenChange={(open) => (open ? undefined : onClose())}
      size="lg"
      title={revision ? `#${revision.revisionNumber}` : ''}
      footer={
        <div className="flex flex-wrap justify-end gap-2">
          <Button variant="outline" disabled={view.busy} onClick={onClose}>{view.labels.close}</Button>
          <Button variant="outline" loading={view.busy} onClick={() => void view.saveDraft()}>{view.labels.saveDraft}</Button>
          <Button variant="primary" loading={view.busy} onClick={() => void view.saveAndSubmit()}>{view.labels.saveAndSubmit}</Button>
        </div>
      }
    >
      <div className="flex flex-col gap-4">
        {view.error ? <p role="alert" className="text-label text-destructive">{view.error}</p> : null}
        {view.notice ? <p role="status" className="text-label text-muted-foreground">{view.notice}</p> : null}
        {view.blockerLabels.length > 0 ? (
          <div>
            <p className="text-label text-muted-foreground">{view.labels.blockers}</p>
            <ul className="mt-1 list-inside list-disc text-body">
              {view.blockerLabels.map((label) => <li key={label}>{label}</li>)}
            </ul>
          </div>
        ) : null}

        <div className="grid grid-cols-1 gap-3 sm:grid-cols-2">
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.nameEn}
            <Input value={view.nameEn} onChange={(event) => view.setNameEn(event.target.value)} aria-label={view.labels.nameEn} />
          </label>
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.nameFr}
            <Input value={view.nameFr} onChange={(event) => view.setNameFr(event.target.value)} aria-label={view.labels.nameFr} />
          </label>
        </div>
        <label className="flex flex-col gap-1 text-label text-muted-foreground">
          {view.labels.descriptionEn}
          <Textarea rows={3} value={view.descriptionEn} onChange={(event) => view.setDescriptionEn(event.target.value)} aria-label={view.labels.descriptionEn} />
        </label>
        <label className="flex flex-col gap-1 text-label text-muted-foreground">
          {view.labels.descriptionFr}
          <Textarea rows={3} value={view.descriptionFr} onChange={(event) => view.setDescriptionFr(event.target.value)} aria-label={view.labels.descriptionFr} />
        </label>

        <div className="grid grid-cols-1 gap-3 sm:grid-cols-2">
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.neighborhood}
            <Input value={view.neighborhood} onChange={(event) => view.setNeighborhood(event.target.value)} aria-label={view.labels.neighborhood} />
          </label>
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.addressLine}
            <Input value={view.addressLine} onChange={(event) => view.setAddressLine(event.target.value)} aria-label={view.labels.addressLine} />
          </label>
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.latitude}
            <Input inputMode="decimal" value={view.latitude} onChange={(event) => view.setLatitude(event.target.value)} aria-label={view.labels.latitude} />
          </label>
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.longitude}
            <Input inputMode="decimal" value={view.longitude} onChange={(event) => view.setLongitude(event.target.value)} aria-label={view.labels.longitude} />
          </label>
        </div>

        <div className="grid grid-cols-1 gap-3 sm:grid-cols-3">
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.checkInFrom}
            <Input type="time" value={view.checkInFrom} onChange={(event) => view.setCheckInFrom(event.target.value)} aria-label={view.labels.checkInFrom} />
          </label>
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.checkInUntil}
            <Input type="time" value={view.checkInUntil} onChange={(event) => view.setCheckInUntil(event.target.value)} aria-label={view.labels.checkInUntil} />
          </label>
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.checkOutBy}
            <Input type="time" value={view.checkOutBy} onChange={(event) => view.setCheckOutBy(event.target.value)} aria-label={view.labels.checkOutBy} />
          </label>
        </div>

        <label className="flex flex-col gap-1 text-label text-muted-foreground">
          {view.labels.houseRulesEn}
          <Textarea rows={2} value={view.houseRulesEn} onChange={(event) => view.setHouseRulesEn(event.target.value)} aria-label={view.labels.houseRulesEn} />
        </label>
        <label className="flex flex-col gap-1 text-label text-muted-foreground">
          {view.labels.houseRulesFr}
          <Textarea rows={2} value={view.houseRulesFr} onChange={(event) => view.setHouseRulesFr(event.target.value)} aria-label={view.labels.houseRulesFr} />
        </label>

        <div className="grid grid-cols-1 gap-3 sm:grid-cols-2">
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.petsPolicy}
            <Select options={view.petsOptions} value={view.petsPolicy} onValueChange={view.setPetsPolicy} ariaLabel={view.labels.petsPolicy} />
          </label>
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.smokingPolicy}
            <Select options={view.smokingOptions} value={view.smokingPolicy} onValueChange={view.setSmokingPolicy} ariaLabel={view.labels.smokingPolicy} />
          </label>
        </div>

        <div className="grid grid-cols-1 gap-3 sm:grid-cols-2">
          <div className="flex flex-col gap-1 text-label text-muted-foreground">
            <span>{view.labels.guestContactPhone}</span>
            <PhoneField value={view.guestContactPhone} onChange={view.setGuestContactPhone} />
          </div>
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.guestContactEmail}
            <Input type="email" value={view.guestContactEmail} onChange={(event) => view.setGuestContactEmail(event.target.value)} aria-label={view.labels.guestContactEmail} />
          </label>
        </div>

        <div>
          <p className="text-label text-muted-foreground">{view.labels.amenities}</p>
          <div className="mt-1 grid grid-cols-1 gap-1 sm:grid-cols-2 lg:grid-cols-3">
            {view.catalog.map((amenity) => (
              <Checkbox key={amenity.code} checked={view.amenities.includes(amenity.code)} onCheckedChange={() => view.toggleAmenity(amenity.code)} label={amenity.name} />
            ))}
          </div>
        </div>
      </div>
    </Dialog>
  );
}
