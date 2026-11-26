import { Ban, CalendarClock, Check, LogIn, LogOut, UserX, X } from 'lucide-react';
import { Button, ConfirmDialog, DateRangePicker, Dialog, Drawer, Skeleton, Textarea } from '@mungini/ui-web';
import { useReservationDetail } from '@/hooks/reservations/useReservationDetail';
import ReservationMessages from '@/components/reservations/ReservationMessages';

type Props = { reservationId: string | null; onClose: () => void; onChanged: () => void };

export default function ReservationDrawer({ reservationId, onClose, onChanged }: Props) {
  const view = useReservationDetail(reservationId, onChanged);
  const detail = view.detail;

  return (
    <>
      <Drawer
        open={reservationId !== null}
        onOpenChange={(open) => (open ? undefined : onClose())}
        title={view.labels.title}
        description={detail?.reference}
        footer={<Button variant="outline" onClick={onClose}>{view.labels.close}</Button>}
      >
        {view.loading && !detail ? <div className="space-y-2"><Skeleton shape="line" /><Skeleton shape="line" /><Skeleton shape="block" /></div> : null}
        {view.loadError ? <p role="alert" className="text-label text-destructive">{view.labels.error}</p> : null}
        {detail ? (
          <div className="flex flex-col gap-4">
            {view.actionError ? <p role="alert" className="text-label text-destructive">{view.actionError}</p> : null}
            {view.notice ? <p role="status" className="text-label text-muted-foreground">{view.notice}</p> : null}
            <dl className="grid grid-cols-1 gap-3 text-body sm:grid-cols-2">
              <div><dt className="text-label text-muted-foreground">{view.labels.reference}</dt><dd className="font-medium">{detail.reference}</dd></div>
              <div><dt className="text-label text-muted-foreground">{view.labels.status}</dt><dd>{view.stateLabel(detail.state)}</dd></div>
              <div><dt className="text-label text-muted-foreground">{view.labels.property}</dt><dd>{detail.propertyName}</dd></div>
              <div><dt className="text-label text-muted-foreground">{view.labels.unit}</dt><dd>{detail.unitTypeName}</dd></div>
              {detail.assignedUnitCode ? <div><dt className="text-label text-muted-foreground">{view.labels.assignedUnit}</dt><dd>{detail.assignedUnitCode}</dd></div> : null}
              <div><dt className="text-label text-muted-foreground">{view.labels.stay}</dt><dd className="tabular-nums">{view.day(detail.checkIn)} – {view.day(detail.checkOut)}</dd></div>
              <div><dt className="text-label text-muted-foreground">{view.labels.adults} / {view.labels.children}</dt><dd className="tabular-nums">{detail.adultCount} / {detail.childCount}</dd></div>
              {detail.approvalDeadlineAt ? <div><dt className="text-label text-muted-foreground">{view.labels.approvalDeadline}</dt><dd className="tabular-nums">{view.day(detail.approvalDeadlineAt)}</dd></div> : null}
              <div><dt className="text-label text-muted-foreground">{view.labels.createdAt}</dt><dd className="tabular-nums">{view.day(detail.createdAt)}</dd></div>
            </dl>

            <div className="border-t border-border pt-3">
              <p className="text-label text-muted-foreground">{view.labels.guest}</p>
              {detail.guest ? (
                <div className="mt-1 flex flex-col gap-1 text-body">
                  <p className="font-medium">{detail.guest.fullName}</p>
                  {detail.guest.contactEmail ? <a className="text-primary hover:underline" href={`mailto:${detail.guest.contactEmail}`}>{detail.guest.contactEmail}</a> : null}
                  {detail.guest.contactPhone ? <a className="text-primary hover:underline" href={`tel:${detail.guest.contactPhone}`}>{detail.guest.contactPhone}</a> : null}
                </div>
              ) : (
                <p className="mt-1 text-label text-muted-foreground">{view.labels.guestHidden}</p>
              )}
            </div>

            {detail.specialRequest ? (
              <div className="border-t border-border pt-3">
                <p className="text-label text-muted-foreground">{view.labels.specialRequest}</p>
                <p className="mt-1 text-body">{detail.specialRequest}</p>
                <p className="mt-1 text-label text-muted-foreground">{view.labels.specialRequestState}: {view.requestStateLabel(detail.specialRequestState)}</p>
                {detail.specialRequestState === 'REQUESTED' && detail.allowedActions.includes('ACCEPT_SPECIAL_REQUEST') ? (
                  <div className="mt-2 flex flex-wrap gap-2">
                    <Button size="sm" variant="primary" icon={<Check />} collapseLabel aria-label={view.labels.acceptSpecialRequest} title={view.labels.acceptSpecialRequest} loading={view.busy === 'ACCEPT_REQUEST'} disabled={view.busy !== null} onClick={view.acceptRequest}>{view.labels.acceptSpecialRequest}</Button>
                    <Button size="sm" variant="outline" icon={<X />} collapseLabel aria-label={view.labels.declineSpecialRequest} title={view.labels.declineSpecialRequest} loading={view.busy === 'DECLINE_REQUEST'} disabled={view.busy !== null} onClick={view.declineRequest}>{view.labels.declineSpecialRequest}</Button>
                  </div>
                ) : null}
              </div>
            ) : null}

            <div className="border-t border-border pt-3">
              <p className="text-label text-muted-foreground">{view.labels.stay}</p>
              <dl className="mt-1 space-y-1 text-body">
                {view.nights.map((night) => (
                  <div key={night.date} className="flex justify-between gap-4">
                    <dt className="text-muted-foreground tabular-nums">{night.date}</dt>
                    <dd className="tabular-nums">{night.amount}</dd>
                  </div>
                ))}
                <div className="flex justify-between gap-4 border-t border-border pt-2 font-medium">
                  <dt>{view.labels.total}</dt>
                  <dd className="tabular-nums">{view.money(detail.total)}</dd>
                </div>
                <div className="flex justify-between gap-4">
                  <dt className="text-muted-foreground">{view.labels.commission} ({view.rate})</dt>
                  <dd className="tabular-nums">{view.money(detail.commissionAmount)}</dd>
                </div>
                <div className="flex justify-between gap-4 font-medium">
                  <dt>{view.labels.net}</dt>
                  <dd className="tabular-nums">{view.money(detail.netAmount)}</dd>
                </div>
              </dl>
            </div>

            <div className="flex flex-col gap-2 border-t border-border pt-3 sm:flex-row sm:flex-wrap">
              {view.can('ACCEPT') ? <Button size="sm" variant="primary" icon={<Check />} collapseLabel aria-label={view.labels.accept} title={view.labels.accept} loading={view.busy === 'ACCEPT'} disabled={view.busy !== null} onClick={view.accept}>{view.labels.accept}</Button> : null}
              {view.can('DECLINE') ? <Button size="sm" variant="outline" icon={<X />} collapseLabel aria-label={view.labels.decline} title={view.labels.decline} disabled={view.busy !== null} onClick={() => view.setConfirmAction('DECLINE')}>{view.labels.decline}</Button> : null}
              {view.can('CHECK_IN') ? <Button size="sm" variant="primary" icon={<LogIn />} collapseLabel aria-label={view.labels.checkIn} title={view.labels.checkIn} loading={view.busy === 'CHECK_IN'} disabled={view.busy !== null} onClick={view.checkIn}>{view.labels.checkIn}</Button> : null}
              {view.can('CHECK_OUT') ? <Button size="sm" variant="primary" icon={<LogOut />} collapseLabel aria-label={view.labels.checkOut} title={view.labels.checkOut} loading={view.busy === 'CHECK_OUT'} disabled={view.busy !== null} onClick={view.checkOut}>{view.labels.checkOut}</Button> : null}
              {view.can('NO_SHOW') ? <Button size="sm" variant="outline" icon={<UserX />} collapseLabel aria-label={view.labels.noShow} title={view.labels.noShow} disabled={view.busy !== null} onClick={() => view.setConfirmAction('NO_SHOW')}>{view.labels.noShow}</Button> : null}
              {view.can('AMEND') ? <Button size="sm" variant="outline" icon={<CalendarClock />} collapseLabel aria-label={view.labels.modifyDates} title={view.labels.modifyDates} disabled={view.busy !== null} onClick={() => view.setAmendOpen(true)}>{view.labels.modifyDates}</Button> : null}
              {view.can('CANCEL') ? <Button size="sm" variant="destructive" icon={<Ban />} collapseLabel aria-label={view.labels.cancelReservation} title={view.labels.cancelReservation} disabled={view.busy !== null} onClick={() => view.setCancelOpen(true)}>{view.labels.cancelReservation}</Button> : null}
            </div>

            <ReservationMessages reservationId={detail.id} enabled={view.canMessage} />
          </div>
        ) : null}
      </Drawer>

      <ConfirmDialog
        open={view.confirmAction !== null}
        onOpenChange={(open) => (open ? undefined : view.setConfirmAction(null))}
        title={view.confirmAction === 'NO_SHOW' ? view.labels.confirmNoShowTitle : view.labels.confirmDeclineTitle}
        description={view.confirmAction === 'NO_SHOW' ? view.labels.confirmNoShowBody : view.labels.confirmDeclineBody}
        confirmLabel={view.confirmAction === 'NO_SHOW' ? view.labels.noShow : view.labels.decline}
        cancelLabel={view.labels.dismiss}
        destructive
        loading={view.busy !== null}
        onConfirm={view.confirmAction === 'NO_SHOW' ? view.noShow : view.decline}
      />

      <Dialog
        open={view.cancelOpen}
        onOpenChange={(open) => (open ? undefined : view.setCancelOpen(false))}
        title={view.labels.cancelTitle}
        description={view.labels.cancelBody}
        footer={
          <div className="flex justify-end gap-2">
            <Button variant="outline" disabled={view.cancelBusy} onClick={() => view.setCancelOpen(false)}>{view.labels.dismiss}</Button>
            <Button variant="destructive" loading={view.cancelBusy} disabled={!view.reason.trim()} onClick={() => void view.submitCancel()}>{view.labels.cancelConfirm}</Button>
          </div>
        }
      >
        <div className="flex flex-col gap-2">
          {view.cancelError ? <p role="alert" className="text-label text-destructive">{view.cancelError}</p> : null}
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.cancelReason}
            <Textarea rows={3} maxLength={300} value={view.reason} placeholder={view.labels.cancelReasonPlaceholder} onChange={(event) => view.setReason(event.target.value)} />
          </label>
        </div>
      </Dialog>

      <Dialog
        open={view.amendOpen}
        onOpenChange={(open) => (open ? undefined : view.setAmendOpen(false))}
        title={view.labels.amendTitle}
        description={view.labels.amendBody}
        footer={
          <div className="flex justify-end gap-2">
            <Button variant="outline" disabled={view.amendBusy} onClick={() => view.setAmendOpen(false)}>{view.labels.dismiss}</Button>
            <Button variant="primary" loading={view.amendBusy} disabled={!view.range.start || !view.range.end} onClick={() => void view.submitAmend()}>{view.labels.amendSubmit}</Button>
          </div>
        }
      >
        <div className="flex flex-col gap-2">
          {view.amendError ? <p role="alert" className="text-label text-destructive">{view.amendError}</p> : null}
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.amendDates}
            <DateRangePicker value={view.range} onChange={view.setRange} ariaLabel={view.labels.amendDates} />
          </label>
        </div>
      </Dialog>
    </>
  );
}
