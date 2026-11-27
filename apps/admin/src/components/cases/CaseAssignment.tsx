import { ChevronLeft, ChevronRight, UserRoundPlus } from 'lucide-react';
import { Button, Dialog, Input, Select } from '@mungini/ui-web';
import { useCaseAssignment } from '@/hooks/cases/useCaseAssignment';
import type { AdminCase } from '@/types/cases/cases';

type Props = { item: AdminCase; onChanged: () => void };

export default function CaseAssignment({ item, onChanged }: Props) {
  const view = useCaseAssignment(item, onChanged);
  return <>
    <Button size="sm" variant="outline" icon={<UserRoundPlus />} collapseLabel aria-label={view.labels.assign} title={view.labels.assign} onClick={() => view.setOpen(true)}>{view.labels.assign}</Button>
    <Dialog open={view.open} onOpenChange={view.setOpen} title={view.labels.assignTitle} footer={<div className="flex justify-end gap-2"><Button variant="outline" onClick={() => view.setOpen(false)} disabled={view.busy}>{view.labels.cancel}</Button><Button loading={view.busy} disabled={!view.selected || view.loading} onClick={() => void view.submit()}>{view.labels.save}</Button></div>}>
      <div className="flex flex-col gap-3">
        {view.error ? <p role="alert" className="text-label text-destructive">{view.error}</p> : null}
        <Input value={view.search} onChange={(event) => view.changeSearch(event.target.value)} placeholder={view.labels.search} aria-label={view.labels.search} />
        <Select options={view.options} value={view.selected} onValueChange={view.setSelected} placeholder={view.labels.staff} ariaLabel={view.labels.staff} disabled={view.loading} />
        <div className="flex items-center justify-end gap-2"><Button size="sm" variant="outline" icon={<ChevronLeft />} aria-label={view.labels.previous} title={view.labels.previous} disabled={view.page <= 1 || view.loading} onClick={() => view.changePage(view.page - 1)} /><span className="text-label tabular-nums">{view.page}</span><Button size="sm" variant="outline" icon={<ChevronRight />} aria-label={view.labels.next} title={view.labels.next} disabled={view.page * 20 >= view.total || view.loading} onClick={() => view.changePage(view.page + 1)} /></div>
      </div>
    </Dialog>
  </>;
}
