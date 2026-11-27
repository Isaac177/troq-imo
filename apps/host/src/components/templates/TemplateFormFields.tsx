import { AmountInput, Input, Select, Switch, Textarea } from '@mungini/ui-web';
import type { TemplateEditorView } from '@/hooks/templates/useTemplateEditor';

type Props = { view: TemplateEditorView };

export default function TemplateFormFields({ view }: Props) {
  return (
    <div className="flex flex-col gap-4">
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

      <div className="grid grid-cols-1 gap-3 sm:grid-cols-2">
        <label className="flex flex-col gap-1 text-label text-muted-foreground">
          {view.labels.descriptionEn}
          <Textarea value={view.descriptionEn} onChange={(event) => view.setDescriptionEn(event.target.value)} aria-label={view.labels.descriptionEn} />
        </label>
        <label className="flex flex-col gap-1 text-label text-muted-foreground">
          {view.labels.descriptionFr}
          <Textarea value={view.descriptionFr} onChange={(event) => view.setDescriptionFr(event.target.value)} aria-label={view.labels.descriptionFr} />
        </label>
      </div>

      <div className="grid grid-cols-1 gap-3 sm:grid-cols-2">
        <label className="flex flex-col gap-1 text-label text-muted-foreground">
          {view.labels.category}
          <Select options={view.categoryOptions} value={view.categoryCode} onValueChange={view.setCategoryCode} ariaLabel={view.labels.category} />
        </label>
        <label className="flex flex-col gap-1 text-label text-muted-foreground">
          {view.labels.inventoryModel}
          <Select options={view.modelOptions} value={view.inventoryModel} onValueChange={view.setInventoryModel} ariaLabel={view.labels.inventoryModel} />
        </label>
      </div>

      <div className="grid grid-cols-2 gap-3 sm:grid-cols-3">
        <label className="flex flex-col gap-1 text-label text-muted-foreground">
          {view.labels.maxAdults}
          <Input inputMode="numeric" value={view.maxAdults} onChange={(event) => view.setMaxAdults(event.target.value)} aria-label={view.labels.maxAdults} />
        </label>
        <label className="flex flex-col gap-1 text-label text-muted-foreground">
          {view.labels.maxChildren}
          <Input inputMode="numeric" value={view.maxChildren} onChange={(event) => view.setMaxChildren(event.target.value)} aria-label={view.labels.maxChildren} />
        </label>
        <label className="flex flex-col gap-1 text-label text-muted-foreground">
          {view.labels.bedrooms}
          <Input inputMode="numeric" value={view.bedrooms} onChange={(event) => view.setBedrooms(event.target.value)} aria-label={view.labels.bedrooms} />
        </label>
        <label className="flex flex-col gap-1 text-label text-muted-foreground">
          {view.labels.beds}
          <Input inputMode="numeric" value={view.beds} onChange={(event) => view.setBeds(event.target.value)} aria-label={view.labels.beds} />
        </label>
        <label className="flex flex-col gap-1 text-label text-muted-foreground">
          {view.labels.bathrooms}
          <Input inputMode="numeric" value={view.bathrooms} onChange={(event) => view.setBathrooms(event.target.value)} aria-label={view.labels.bathrooms} />
        </label>
      </div>

      <div className="grid grid-cols-1 gap-3 sm:grid-cols-2">
        <label className="flex flex-col gap-1 text-label text-muted-foreground">
          {view.labels.baseAmount}
          <AmountInput value={view.baseNightlyAmount} onValueChange={view.setBaseNightlyAmount} fractionDigits={2} ariaLabel={view.labels.baseAmount} />
          <span className="text-micro text-muted-foreground">{view.labels.amountHint}</span>
        </label>
        <label className="flex flex-col gap-1 text-label text-muted-foreground">
          {view.labels.status}
          <Select options={view.statusOptions} value={view.status} onValueChange={view.setStatus} ariaLabel={view.labels.status} />
        </label>
      </div>

      <div className="flex flex-col gap-2" role="group" aria-label={view.labels.accessibility}>
        <p className="text-label text-muted-foreground">{view.labels.accessibility}</p>
        <div className="flex flex-wrap gap-x-4 gap-y-2">
          {view.accessibilityOptions.map((option) => (
            <Switch key={option.code} checked={option.checked} onCheckedChange={option.toggle} label={option.label} />
          ))}
        </div>
      </div>
    </div>
  );
}
