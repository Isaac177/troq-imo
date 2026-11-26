'use client';

import { Button, ConfirmDialog, Dialog, cn } from '@mungini/ui-web';
import { textTones } from '@mungini/theme';
import type { TripActionsView } from '@/hooks/trips/useTripActions';

type Props = { view: TripActionsView };

export const TripActions = ({ view }: Props) => (
  <div className="flex flex-col gap-3">
    {view.cancelled && (
      <div role="status" className="animate-fade-in rounded-card border border-border bg-card p-3 text-body">
        <p className="font-medium">{view.labels.cancelSuccess}</p>
        <dl className="mt-2 divide-y divide-border">
          <div className="flex justify-between gap-4 py-1.5">
            <dt className="text-muted-foreground">{view.labels.chargeDue}</dt>
            <dd className="tabular-nums">{view.chargeDue}</dd>
          </div>
          {view.refunded && (
            <div className="flex justify-between gap-4 py-1.5">
              <dt className="text-muted-foreground">{view.labels.refundNote}</dt>
              <dd className="tabular-nums">{view.refund}</dd>
            </div>
          )}
        </dl>
      </div>
    )}
    {view.withdrawn && (
      <div role="status" className="animate-fade-in rounded-card border border-border bg-card p-3 text-body">
        <p className="font-medium">{view.labels.withdrawSuccess}</p>
      </div>
    )}
    {view.canCancel && !view.completed && (
      <Button variant="destructive" className="w-full sm:w-auto" onClick={view.openCancel}>
        {view.labels.cancel}
      </Button>
    )}
    {view.canWithdraw && !view.completed && (
      <Button variant="destructive" className="w-full sm:w-auto" onClick={view.openWithdraw}>
        {view.labels.withdraw}
      </Button>
    )}
    {view.inlineError && <p role="alert" className={cn('text-body', textTones.problem)}>{view.inlineError}</p>}
    <Dialog
      open={view.cancelOpen}
      onOpenChange={view.onOpenChange}
      title={view.labels.cancelTitle}
      description={view.labels.cancelIntro}
      size="sm"
      footer={
        <>
          <Button variant="outline" disabled={view.busy} onClick={view.close}>
            {view.labels.keepTrip}
          </Button>
          <Button variant="destructive" loading={view.busy} disabled={!view.cancelReady} onClick={view.confirmCancel}>
            {view.labels.confirmCancel}
          </Button>
        </>
      }
    >
      {view.previewLoading && <p className="text-body text-muted-foreground">{view.labels.termsLoading}</p>}
      {view.previewError && (
        <div role="alert" className={cn('flex flex-wrap items-center gap-3 text-body', textTones.problem)}>
          <span>{view.previewError}</span>
          <Button size="sm" variant="outline" onClick={view.retryPreview}>
            {view.labels.retry}
          </Button>
        </div>
      )}
      {view.preview && (
        <dl className="divide-y divide-border text-body">
          <div className="flex justify-between gap-4 py-2">
            <dt className="text-muted-foreground">{view.labels.total}</dt>
            <dd className="tabular-nums">{view.money(view.preview.total)}</dd>
          </div>
          <div className="flex justify-between gap-4 py-2">
            <dt className="text-muted-foreground">{view.labels.refundLabel}</dt>
            <dd className="tabular-nums">{view.money(view.preview.refund)}</dd>
          </div>
          <div className="flex justify-between gap-4 py-2">
            <dt className="text-muted-foreground">{view.labels.penalty}</dt>
            <dd className={cn('tabular-nums', view.hasPenalty && textTones.problem)}>
              {view.penaltyText}
            </dd>
          </div>
          <div className="flex flex-col gap-1 py-2 sm:flex-row sm:items-center sm:justify-between">
            <dt className="text-muted-foreground">{view.labels.deadlineLabel}</dt>
            <dd className={cn('tabular-nums', textTones.attention)}>{view.deadline}</dd>
          </div>
        </dl>
      )}
      {view.actionError && <p role="alert" className={cn('mt-2 text-body', textTones.problem)}>{view.actionError}</p>}
    </Dialog>
    <ConfirmDialog
      open={view.withdrawOpen}
      onOpenChange={view.onOpenChange}
      title={view.labels.withdrawTitle}
      description={view.withdrawDescription}
      confirmLabel={view.labels.confirmWithdraw}
      cancelLabel={view.labels.keepRequest}
      destructive
      loading={view.busy}
      onConfirm={view.confirmWithdraw}
    />
  </div>
);
