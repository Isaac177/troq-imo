import { CheckCircle2 } from 'lucide-react';
import { Button, Drawer, Skeleton, StatusDot } from '@mungini/ui-web';
import { textTones } from '@mungini/theme/tones';
import CaseCloseDialog from '@/components/cases/CaseCloseDialog';
import CaseEvidenceList from '@/components/cases/CaseEvidenceList';
import CaseEvidencePreview from '@/components/cases/CaseEvidencePreview';
import { useCaseDetail } from '@/hooks/cases/useCaseDetail';
import { useCaseEvidence } from '@/hooks/cases/useCaseEvidence';
import { CASE_STATE_TONES } from '@/types/cases/cases';

type Props = {
  caseId: string | null;
  canWrite: boolean;
  onClose: () => void;
  onChanged: () => void;
};

export default function CaseDetailDrawer({ caseId, canWrite, onClose, onChanged }: Props) {
  const detail = useCaseDetail(caseId, canWrite, onChanged);
  const evidence = useCaseEvidence(caseId, detail.canAct, onChanged);
  const item = detail.detail;

  return (
    <>
      <Drawer
        open={caseId !== null}
        onOpenChange={(open) => (open ? undefined : onClose())}
        title={item ? detail.typeLabel(item.caseType) : detail.labels.loading}
        description={item ? item.reservationReference ?? detail.labels.noReservation : undefined}
        footer={
          <div className="flex justify-end gap-2">
            <Button variant="outline" size="sm" onClick={onClose}>{detail.labels.close}</Button>
            {detail.canAct ? <Button variant="primary" size="sm" icon={<CheckCircle2 />} onClick={detail.openClose}>{detail.labels.closeCase}</Button> : null}
          </div>
        }
      >
        {detail.loading && !item ? <div className="space-y-2"><Skeleton shape="line" /><Skeleton shape="block" /></div> : null}
        {detail.error ? <p role="alert" className="text-label text-destructive">{detail.error}</p> : null}
        {detail.notice ? <p role="status" className="text-label text-muted-foreground">{detail.notice}</p> : null}
        {item ? (
          <div className="flex flex-col gap-4">
            <dl className="grid grid-cols-1 gap-3 text-body sm:grid-cols-2">
              <div>
                <dt className="text-label text-muted-foreground">{detail.labels.state}</dt>
                <dd><StatusDot tone={CASE_STATE_TONES[item.state] ?? 'neutral'} label={detail.stateLabel(item.state)} /></dd>
              </div>
              <div>
                <dt className="text-label text-muted-foreground">{detail.labels.reservation}</dt>
                <dd>{item.reservationReference ?? detail.labels.noReservation}</dd>
              </div>
              <div>
                <dt className="text-label text-muted-foreground">{detail.labels.opened}</dt>
                <dd className="tabular-nums">{detail.openedText}</dd>
              </div>
              <div>
                <dt className="text-label text-muted-foreground">{detail.labels.deadline}</dt>
                <dd className={`tabular-nums ${textTones.attention}`}>{detail.deadlineText}</dd>
              </div>
            </dl>

            <div className="border-t border-border pt-3">
              <p className="text-label text-muted-foreground">{detail.labels.reason}</p>
              <p className="mt-1 whitespace-pre-wrap text-body text-foreground">{item.reason}</p>
            </div>

            {detail.resolutionText ? (
              <div className="border-t border-border pt-3">
                <p className="text-label text-muted-foreground">{detail.labels.resolution}</p>
                <p className="mt-1 whitespace-pre-wrap text-body text-foreground">{detail.resolutionText}</p>
              </div>
            ) : null}

            <div className="border-t border-border pt-3">
              <CaseEvidenceList view={evidence} />
            </div>
          </div>
        ) : null}
      </Drawer>
      <CaseCloseDialog view={detail} />
      <CaseEvidencePreview view={evidence} />
    </>
  );
}
