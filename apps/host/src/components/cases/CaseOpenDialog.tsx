import { AmountInput, Button, Dialog, SearchableSelect, Select, Textarea } from '@mungini/ui-web';
import type { OpenCaseView } from '@/hooks/cases/useOpenCase';

type Props = {
  view: OpenCaseView;
};

export default function CaseOpenDialog({ view }: Props) {
  return (
    <Dialog
      open={view.open}
      onOpenChange={view.onOpenChange}
      title={view.labels.title}
      description={view.labels.description}
      footer={
        <div className="flex justify-end gap-2">
          <Button variant="outline" size="sm" disabled={view.busy} onClick={view.close}>{view.labels.cancel}</Button>
          <Button variant="primary" size="sm" loading={view.busy} disabled={!view.canSubmit} onClick={() => void view.submit()}>{view.labels.submit}</Button>
        </div>
      }
    >
      <div className="flex flex-col gap-3">
        {view.formError ? <p role="alert" className="text-label text-destructive">{view.formError}</p> : null}
        <div className="flex flex-col gap-1 text-label text-muted-foreground">
          <span>{view.labels.reservation}</span>
          <SearchableSelect
            options={view.reservationOptions}
            value={view.reservationId}
            onValueChange={view.setReservationId}
            placeholder={view.labels.selectReservation}
            searchPlaceholder={view.labels.searchReservation}
            emptyLabel={view.labels.noReservations}
            loading={view.loadingReservations}
            ariaLabel={view.labels.reservation}
          />
          {view.reservationsError ? <span role="alert" className="text-destructive">{view.reservationsError}</span> : null}
        </div>
        <label className="flex flex-col gap-1 text-label text-muted-foreground">
          {view.labels.caseType}
          <Select options={view.typeOptions} value={view.caseType} onValueChange={view.selectCaseType} ariaLabel={view.labels.caseType} />
        </label>
        <label className="flex flex-col gap-1 text-label text-muted-foreground">
          {view.labels.reason}
          <Textarea rows={4} maxLength={2000} value={view.reason} placeholder={view.labels.reasonPlaceholder} onChange={(event) => view.setReason(event.target.value)} aria-label={view.labels.reason} />
        </label>
        {view.caseType === 'DAMAGE_CLAIM' ? (
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.claimAmount}
            <AmountInput value={view.claimAmount} onValueChange={view.setClaimAmount} fractionDigits={2} placeholder={view.labels.amountPlaceholder} ariaLabel={view.labels.claimAmount} />
            <span>{view.labels.claimAmountHint}</span>
          </label>
        ) : null}
      </div>
    </Dialog>
  );
}
