type Options = {
  page: number;
  pageSize: number;
  total: number;
  onPageChange: (page: number) => void;
};

export const usePagination = ({ page, pageSize, total, onPageChange }: Options) => {
  const pageCount = Math.max(1, Math.ceil(total / Math.max(pageSize, 1)));
  return {
    pageCount,
    canPrevious: page > 1,
    canNext: page < pageCount,
    previous: () => onPageChange(Math.max(1, page - 1)),
    next: () => onPageChange(Math.min(pageCount, page + 1)),
  };
};
