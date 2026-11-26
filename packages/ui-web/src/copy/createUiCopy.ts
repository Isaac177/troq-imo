import type { UiCopy } from './UiCopy';

export const createUiCopy = (translate: (key: string) => string, locale: string): UiCopy => ({
  locale,
  common: {
    close: translate('close'),
    back: translate('back'),
    cancel: translate('cancel'),
    confirm: translate('confirm'),
    clear: translate('clear'),
    search: translate('search'),
    loading: translate('loading'),
    noResults: translate('noResults'),
    select: translate('select'),
    more: translate('more'),
  },
  pagination: {
    count: (total) => translate('count').replace('{total}', String(total)),
    page: (page, pages) => translate('page').replace('{page}', String(page)).replace('{pages}', String(pages)),
    previous: translate('previous'),
    next: translate('next'),
  },
  table: {
    selectAll: translate('selectAll'),
    selectRow: (rowNumber) => translate('selectRow').replace('{row}', String(rowNumber)),
    sortedAscending: translate('sortedAscending'),
    sortedDescending: translate('sortedDescending'),
    sortBy: (column) => translate('sortBy').replace('{column}', column),
    openRecord: translate('openRecord'),
    error: translate('error'),
    empty: translate('empty'),
  },
  bulk: {
    selected: (count) => translate('selected').replace('{count}', String(count)),
    clearSelection: translate('clearSelection'),
  },
  date: {
    pick: translate('pick'),
    pickRange: translate('pickRange'),
    pickDateTime: translate('pickDateTime'),
    previousMonth: translate('previousMonth'),
    nextMonth: translate('nextMonth'),
    today: translate('today'),
    hours: translate('hours'),
    minutes: translate('minutes'),
    rangeStart: translate('rangeStart'),
    rangeEnd: translate('rangeEnd'),
  },
  phone: {
    country: translate('country'),
    countrySearch: translate('countrySearch'),
    countryEmpty: translate('countryEmpty'),
  },
  toast: {
    dismiss: translate('dismiss'),
  },
  richText: {
    placeholder: translate('placeholder'),
  },
});
