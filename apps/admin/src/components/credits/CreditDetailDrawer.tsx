import { Minus, Percent, Plus } from 'lucide-react';
import { Button, Drawer, EmptyState, Skeleton, StatusDot } from '@mungini/ui-web';
import CreditEntryDialog from '@/components/credits/CreditEntryDialog';
import CreditLedger from '@/components/credits/CreditLedger';
import CommissionRateDialog from '@/components/credits/CommissionRateDialog';
import CommissionRateHistory from '@/components/credits/CommissionRateHistory';
import { useCreditDetail } from '@/hooks/credits/useCreditDetail';

type Props = { hostId: string | null; hostName: string; onClose: () => void; onChanged: () => void };

export default function CreditDetailDrawer({ hostId, hostName, onClose, onChanged }: Props) {
  const view = useCreditDetail(hostId, onChanged);
  const item = view.detail;
  return (
    <>
      <Drawer
        open={hostId !== null}
        onOpenChange={(open) => (open ? undefined : onClose())}
        title={hostName || view.labels.title}
        description={view.labels.title}
        footer={<Button variant="outline" onClick={onClose}>{view.labels.close}</Button>}
      >
        {!item && !view.error && !view.forbidden ? <div className="space-y-2"><Skeleton shape="line" /><Skeleton shape="line" /><Skeleton shape="block" /></div> : null}
        {view.error && !view.forbidden ? <p role="alert" className="text-label text-destructive">{view.error}</p> : null}
        {view.forbidden ? <EmptyState title={view.labels.forbidden} /> : null}
        {view.notice ? <p role="status" className="text-label text-muted-foreground">{view.notice}</p> : null}
        {item ? (
          <div className="flex flex-col gap-4">
            <dl className="grid grid-cols-1 gap-3 text-body sm:grid-cols-2">
              <div>
                <dt className="text-label text-muted-foreground">{view.labels.balance}</dt>
                <dd className="flex flex-wrap items-center gap-2">
                  <span className="tabular-nums text-title font-semibold text-foreground">{view.money(item.balance)}</span>
                  <StatusDot tone={view.statusTone} label={view.stateLabel(view.status)} />
                </dd>
              </div>
              <div>
                <dt className="text-label text-muted-foreground">{view.labels.lowThreshold}</dt>
                <dd className="tabular-nums">{view.money(item.lowThreshold)}</dd>
              </div>
              <div>
                <dt className="text-label text-muted-foreground">{view.labels.commissionRate}</dt>
                <dd className="tabular-nums">{view.percent(item.rateBps)}</dd>
                {view.rate ? (
                  <p className="mt-0.5 text-micro text-muted-foreground">
                    {view.rate.configured && view.rate.effectiveFrom ? `${view.labels.activeSince} ${view.time(view.rate.effectiveFrom)}` : view.labels.platformDefault}
                  </p>
                ) : null}
              </div>
            </dl>

            <div className="flex flex-col gap-2 border-t border-border pt-3 sm:flex-row">
              <Button size="sm" variant="primary" icon={<Plus />} collapseLabel onClick={view.openTopup}>{view.labels.topup}</Button>
              <Button size="sm" variant="outline" icon={<Minus />} collapseLabel onClick={view.openAdjust}>{view.labels.adjust}</Button>
              <Button size="sm" variant="outline" icon={<Percent />} collapseLabel onClick={view.openRate}>{view.labels.setRate}</Button>
            </div>

            <CommissionRateHistory key={`rate-${hostId ?? 'none'}`} hostId={hostId} reloadToken={view.reloadToken} timezone={view.timezone} />
            <CreditLedger key={`ledger-${hostId ?? 'none'}`} hostId={hostId} reloadToken={view.reloadToken} timezone={view.timezone} />
          </div>
        ) : null}
      </Drawer>

      <CreditEntryDialog hostId={hostId} mode="topup" open={view.dialog === 'topup'} onOpenChange={(open) => (open ? undefined : view.closeDialog())} onSaved={view.markSaved} />
      <CreditEntryDialog hostId={hostId} mode="adjust" open={view.dialog === 'adjust'} onOpenChange={(open) => (open ? undefined : view.closeDialog())} onSaved={view.markSaved} />
      <CommissionRateDialog hostId={hostId} currentBps={view.currentBps} open={view.dialog === 'rate'} onOpenChange={(open) => (open ? undefined : view.closeDialog())} onSaved={view.markSaved} />
    </>
  );
}
