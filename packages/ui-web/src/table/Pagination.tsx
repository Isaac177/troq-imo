import { ChevronLeft, ChevronRight } from 'lucide-react';
import { IconTooltipButton } from '../button/IconTooltipButton';
import { useUiCopy } from '../copy/useUiCopy';
import { cn } from '../lib/cn';
import { usePagination } from './usePagination';

type PaginationProps = {
  page: number;
  pageSize: number;
  total: number;
  onPageChange: (page: number) => void;
  className?: string;
};

export const Pagination = ({ page, pageSize, total, onPageChange, className }: PaginationProps) => {
  const { pagination } = useUiCopy();
  const view = usePagination({ page, pageSize, total, onPageChange });
  return (
    <div className={cn('flex items-center justify-between gap-3 border-t border-border px-3 py-2.5 md:px-4', className)}>
      <p className="text-body tabular-nums text-muted-foreground">{pagination.count(total)}</p>
      <div className="flex items-center gap-1">
        <IconTooltipButton icon={<ChevronLeft />} label={pagination.previous} onClick={view.previous} disabled={!view.canPrevious} />
        <span className="px-1 text-body tabular-nums text-muted-foreground">{pagination.page(page, view.pageCount)}</span>
        <IconTooltipButton icon={<ChevronRight />} label={pagination.next} onClick={view.next} disabled={!view.canNext} />
      </div>
    </div>
  );
};
