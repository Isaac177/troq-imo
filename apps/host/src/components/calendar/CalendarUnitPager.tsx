import { Button } from '@mungini/ui-web';

type Props = {
  page: number;
  pages: number;
  label?: string;
  previousLabel: string;
  nextLabel: string;
  onPrevious: () => void;
  onNext: () => void;
};

export default function CalendarUnitPager({ page, pages, label, previousLabel, nextLabel, onPrevious, onNext }: Props) {
  return (
    <div className="flex flex-wrap items-center gap-2 text-label text-muted-foreground">
      {label ? <span>{label}</span> : null}
      <Button variant="ghost" size="sm" disabled={page <= 1} onClick={onPrevious}>{previousLabel}</Button>
      <span className="tabular-nums">{page} / {pages}</span>
      <Button variant="ghost" size="sm" disabled={page >= pages} onClick={onNext}>{nextLabel}</Button>
    </div>
  );
}
