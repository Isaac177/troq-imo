import { Banknote, Gavel } from 'lucide-react';
import { AmountInput, Button, Dialog, Drawer, Input, Select, Skeleton } from '@mungini/ui-web';
import { useClaimDetail } from '@/hooks/claims/useClaimDetail';

type Props = { caseId: string | null; onClose: () => void; onChanged: () => void };

export default function ClaimDetailDrawer({ caseId, onClose, onChanged }: Props) {
  const view = useClaimDetail(caseId, onChanged);
  const item = view.detail;
  return (
    <>
      <Drawer open={caseId !== null} onOpenChange={(open) => (open ? undefined : onClose())} title={view.labels.decide} description={item?.reservationReference ?? ''} footer={<Button variant="outline" onClick={onClose}>{view.labels.close}</Button>}>
        {view.loading && !item ? <div className="space-y-2"><Skeleton shape="line" /><Skeleton shape="line" /><Skeleton shape="block" /></div> : null}
        {view.error ? <p role="alert" className="text-label text-destructive">{view.labels.error}</p> : null}
        {view.notice ? <p role="status" className="text-label text-muted-foreground">{view.notice}</p> : null}
        {item ? (
          <div className="flex flex-col gap-4">
            <dl className="grid grid-cols-1 gap-3 text-body sm:grid-cols-2">
              <div><dt className="text-label text-muted-foreground">{view.labels.property}</dt><dd>{view.propertyName}</dd></div>
              <div><dt className="text-label text-muted-foreground">{view.labels.host}</dt><dd>{item.operatorName}</dd></div>
              <div><dt className="text-label text-muted-foreground">{view.labels.guest}</dt><dd>{item.guestName ?? '—'}</dd></div>
              <div><dt className="text-label text-muted-foreground">{view.labels.reference}</dt><dd>{item.reservationReference ?? '—'}</dd></div>
              <div><dt className="text-label text-muted-foreground">{view.labels.claimAmount}</dt><dd className="tabular-nums">{view.money(item.claimAmount)}</dd></div>
              <div><dt className="text-label text-muted-foreground">{view.labels.stayTotal}</dt><dd className="tabular-nums">{view.money(item.stayTotal)}</dd></div>
              <div><dt className="text-label text-muted-foreground">{view.labels.checkIn} / {view.labels.checkOut}</dt><dd className="tabular-nums">{item.checkIn ?? '—'} / {item.checkOut ?? '—'}</dd></div>
              <div><dt className="text-label text-muted-foreground">{view.labels.deadline}</dt><dd className="tabular-nums">{view.time(item.deadlineAt)}</dd></div>
            </dl>

            <div className="border-t border-border pt-3">
              <p className="text-label text-muted-foreground">{view.labels.reason}</p>
              <p className="mt-1 whitespace-pre-wrap text-body text-foreground">{item.reason}</p>
            </div>

            <div className="border-t border-border pt-3">
              <p className="text-label text-muted-foreground">{view.labels.evidence}</p>
              {item.evidence.length === 0 ? <p className="mt-1 text-label text-muted-foreground">{view.labels.noEvidence}</p> : null}
              <ul className="mt-1 divide-y divide-border">
                {item.evidence.map((evidence) => (
                  <li key={evidence.id} className="flex items-center justify-between gap-2 py-2 text-body">
                    <span className="min-w-0 truncate">{evidence.fileName}</span>
                    <span className="text-label tabular-nums text-muted-foreground">{Math.max(1, Math.ceil(evidence.sizeBytes / 1024))} KB</span>
                  </li>
                ))}
              </ul>
            </div>

            {item.decision ? (
              <div className="border-t border-border pt-3">
                <p className="text-label text-muted-foreground">{view.labels.decision}</p>
                <dl className="mt-1 space-y-1 text-body">
                  <div className="flex justify-between gap-4"><dt className="text-muted-foreground">{view.labels.approvedAmount}</dt><dd className="tabular-nums">{view.money(item.decision.approvedAmount)}</dd></div>
                  <div className="flex justify-between gap-4"><dt className="text-muted-foreground">{view.labels.reasons}</dt><dd>{view.reasonLabel(item.decision.reasonCode)}</dd></div>
                  <div className="flex justify-between gap-4"><dt className="text-muted-foreground">{view.labels.payout}</dt><dd>{view.payoutLabel(item.decision.payoutState)}</dd></div>
                  {item.decision.payoutRef ? <div className="flex justify-between gap-4"><dt className="text-muted-foreground">{view.labels.payoutRef}</dt><dd className="truncate">{item.decision.payoutRef}</dd></div> : null}
                  <div className="flex justify-between gap-4"><dt className="text-muted-foreground">{view.labels.decide}</dt><dd className="tabular-nums">{view.time(item.decision.decidedAt)}</dd></div>
                </dl>
              </div>
            ) : null}

            <div className="flex flex-col gap-2 border-t border-border pt-3 sm:flex-row">
              {view.canDecide ? <Button size="sm" variant="primary" icon={<Gavel />} onClick={view.openDecision}>{view.labels.decide}</Button> : null}
              {view.canPayout ? <Button size="sm" variant="outline" icon={<Banknote />} onClick={view.openPayout}>{view.labels.payout}</Button> : null}
            </div>
          </div>
        ) : null}
      </Drawer>

      <Dialog
        open={view.decisionOpen}
        onOpenChange={view.setDecisionOpen}
        title={view.labels.decisionTitle}
        description={view.labels.decisionBody}
        footer={
          <div className="flex justify-end gap-2">
            <Button variant="outline" disabled={view.busy} onClick={() => view.setDecisionOpen(false)}>{view.labels.dismiss}</Button>
            <Button variant={view.decision === 'REJECTED' ? 'destructive' : 'primary'} loading={view.busy} disabled={!view.reasonCode || (view.decision === 'PARTIAL' && !view.approvedAmount.trim())} onClick={() => void view.submitDecision()}>{view.labels.save}</Button>
          </div>
        }
      >
        <div className="flex flex-col gap-3">
          {view.formError ? <p role="alert" className="text-label text-destructive">{view.formError}</p> : null}
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.decision}
            <Select options={view.decisionOptions} value={view.decision} onValueChange={view.changeDecision} ariaLabel={view.labels.decision} />
          </label>
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.reasons}
            <Select options={view.reasonOptions} value={view.reasonCode} onValueChange={view.setReasonCode} ariaLabel={view.labels.reasons} />
          </label>
          {view.decision === 'PARTIAL' ? (
            <label className="flex flex-col gap-1 text-label text-muted-foreground">
              {view.labels.partialAmount}
              <AmountInput value={view.approvedAmount} onValueChange={view.setApprovedAmount} fractionDigits={2} ariaLabel={view.labels.partialAmount} />
            </label>
          ) : null}
        </div>
      </Dialog>

      <Dialog
        open={view.payoutOpen}
        onOpenChange={view.setPayoutOpen}
        title={view.labels.payoutTitle}
        description={view.labels.payoutBody}
        footer={
          <div className="flex justify-end gap-2">
            <Button variant="outline" disabled={view.busy} onClick={() => view.setPayoutOpen(false)}>{view.labels.dismiss}</Button>
            <Button variant="primary" loading={view.busy} disabled={view.payoutState === 'PAID' && !view.payoutRef.trim()} onClick={() => void view.submitPayout()}>{view.labels.save}</Button>
          </div>
        }
      >
        <div className="flex flex-col gap-3">
          {view.formError ? <p role="alert" className="text-label text-destructive">{view.formError}</p> : null}
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.payoutState}
            <Select options={view.payoutOptions} value={view.payoutState} onValueChange={view.setPayoutState} ariaLabel={view.labels.payoutState} />
          </label>
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.payoutRef}
            <Input value={view.payoutRef} onChange={(event) => view.setPayoutRef(event.target.value)} aria-label={view.labels.payoutRef} />
            <span className="text-micro">{view.labels.payoutRefHint}</span>
          </label>
        </div>
      </Dialog>
    </>
  );
}
