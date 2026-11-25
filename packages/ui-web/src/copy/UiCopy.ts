export type UiCopy = {
  locale: string;
  common: {
    close: string;
    back: string;
    cancel: string;
    confirm: string;
    clear: string;
    search: string;
    loading: string;
    noResults: string;
    select: string;
    more: string;
  };
  pagination: {
    count: (total: number) => string;
    page: (page: number, pages: number) => string;
    previous: string;
    next: string;
  };
  table: {
    selectAll: string;
    selectRow: (rowNumber: number) => string;
    sortedAscending: string;
    sortedDescending: string;
    sortBy: (column: string) => string;
    openRecord: string;
    error: string;
    empty: string;
  };
  bulk: {
    selected: (count: number) => string;
    clearSelection: string;
  };
  date: {
    pick: string;
    pickRange: string;
    pickDateTime: string;
    previousMonth: string;
    nextMonth: string;
    today: string;
    hours: string;
    minutes: string;
    rangeStart: string;
    rangeEnd: string;
  };
  phone: {
    country: string;
    countrySearch: string;
    countryEmpty: string;
  };
  toast: {
    dismiss: string;
  };
  richText: {
    placeholder: string;
  };
};
