import { useEffect, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { Button, type TableColumn } from '@mungini/ui-web';
import { formatCdf, formatIsoDay } from '@mungini/utils';
import { createRule, listRules, removeRule } from '@/services/rates/rates.api';
import type { RatePlan, RateRule } from '@/types/rates/rates';

const PAGE_SIZE = 20;
const WEEKDAY_KEYS = ['mon', 'tue', 'wed', 'thu', 'fri', 'sat', 'sun'];
const KNOWN_ERRORS = ['FORBIDDEN', 'CHOICE_INVALID', 'PERCENT_OUT_OF_RANGE', 'BASE_RATE_NOT_POSITIVE', 'BASE_RATE_OVERLAP', 'BASE_RATE_CONFLICT', 'PERIOD_INVALID', 'WEEKDAYS_INVALID', 'PRIORITY_OUT_OF_RANGE', 'RATE_RULE_NOT_FOUND', 'RATE_PLAN_CHANGED'];

export const useRateRules = (propertyId: string, plan: RatePlan | null, canWrite: boolean) => {
  const { t, i18n } = useTranslation('hostRates');
  const translate = t as unknown as (key: string) => string;
  const { client } = useSession();
  const [rows, setRows] = useState<RateRule[]>([]);
  const [total, setTotal] = useState(0);
  const [page, setPage] = useState(1);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [notice, setNotice] = useState<string | null>(null);
  const [createOpen, setCreateOpen] = useState(false);
  const [startDate, setStartDate] = useState<string | null>(null);
  const [endDate, setEndDate] = useState<string | null>(null);
  const [weekdays, setWeekdays] = useState<number[]>([1, 2, 3, 4, 5, 6, 7]);
  const [adjustmentType, setAdjustmentType] = useState('OVERRIDE');
  const [amount, setAmount] = useState('');
  const [priority, setPriority] = useState(0);
  const [busy, setBusy] = useState(false);
  const [formError, setFormError] = useState<string | null>(null);
  const [removeTarget, setRemoveTarget] = useState<RateRule | null>(null);
  const planId = plan?.id ?? '';

  const message = (code: string | null) => (code && KNOWN_ERRORS.includes(code) ? translate(`errors.${code}`) : translate('errors.generic'));

  useEffect(() => {
    setPage(1);
    setRows([]);
    setTotal(0);
    setNotice(null);
    setCreateOpen(false);
  }, [planId]);

  useEffect(() => {
    if (!propertyId || !planId) return;
    let active = true;
    setLoading(true);
    setError(null);
    void listRules(client, propertyId, planId, page, PAGE_SIZE).then((result) => {
      if (!active) return;
      setLoading(false);
      if (result.error || !result.data) {
        setError(message(result.error));
        setRows([]);
        setTotal(0);
        return;
      }
      setRows(result.data.items);
      setTotal(result.data.total);
    });
    return () => {
      active = false;
    };
  }, [client, propertyId, planId, page, translate]);

  const openCreate = () => {
    setStartDate(null);
    setEndDate(null);
    setWeekdays([1, 2, 3, 4, 5, 6, 7]);
    setAdjustmentType('OVERRIDE');
    setAmount('');
    setPriority(0);
    setFormError(null);
    setCreateOpen(true);
  };

  const toggleWeekday = (day: number) => setWeekdays((current) => (current.includes(day) ? current.filter((value) => value !== day) : [...current, day].sort((left, right) => left - right)));

  const submit = async () => {
    if (!propertyId || !planId) return;
    setBusy(true);
    setFormError(null);
    const result = await createRule(client, propertyId, planId, { startDate, endDate, weekdays, adjustmentType, amount, priority });
    setBusy(false);
    if (result.error || !result.data) {
      setFormError(message(result.error));
      return;
    }
    setCreateOpen(false);
    setNotice(translate('ruleCreated'));
    setRows((current) => [result.data as RateRule, ...current]);
    setTotal((current) => current + 1);
  };

  const confirmRemove = async () => {
    if (!propertyId || !planId || !removeTarget) return;
    setBusy(true);
    setError(null);
    const result = await removeRule(client, propertyId, planId, removeTarget.id);
    setBusy(false);
    if (result.error) {
      setError(message(result.error));
      return;
    }
    setRows((current) => current.filter((rule) => rule.id !== removeTarget.id));
    setTotal((current) => Math.max(0, current - 1));
    setRemoveTarget(null);
    setNotice(translate('ruleRemoved'));
  };

  const weekdayLabel = (day: number) => translate(`weekdaysShort.${WEEKDAY_KEYS[day - 1]}`);
  const columns: TableColumn<RateRule>[] = [
    {
      key: 'dates',
      header: translate('startDate'),
      cell: (row) => <span className="whitespace-nowrap tabular-nums">{row.startDate ? formatIsoDay(row.startDate) : '—'} – {row.endDate ? formatIsoDay(row.endDate) : '—'}</span>,
    },
    { key: 'weekdays', header: translate('weekdays'), hideBelow: 'sm', cell: (row) => row.weekdays.map(weekdayLabel).join(' ') },
    { key: 'adjustment', header: translate('adjustment'), cell: (row) => <div className="min-w-0"><p>{translate(`adjustments.${row.adjustmentType}`)}</p><p className="text-label tabular-nums text-muted-foreground">{row.adjustmentType === 'PERCENTAGE' ? `${Number(row.amount).toFixed(2)}%` : formatCdf(row.amount, i18n.language)}</p></div> },
    { key: 'priority', header: translate('priority'), align: 'right', hideBelow: 'sm', cell: (row) => <span className="tabular-nums">{row.priority}</span> },
    { key: 'actions', header: '', align: 'right', cell: (row) => (canWrite ? <Button size="sm" variant="outline" onClick={() => setRemoveTarget(row)}>{translate('removeRule')}</Button> : null) },
  ];

  return {
    rows, total, page, setPage, pageSize: PAGE_SIZE, loading, error, notice, columns,
    createOpen, setCreateOpen, openCreate, startDate, setStartDate, endDate, setEndDate,
    weekdays, toggleWeekday, adjustmentType, setAdjustmentType, amount, setAmount, priority, setPriority,
    busy, formError, submit, removeTarget, setRemoveTarget, confirmRemove,
    adjustmentOptions: ['OVERRIDE', 'AMOUNT', 'PERCENTAGE'].map((value) => ({ value, label: translate(`adjustments.${value}`) })),
    weekdayOptions: WEEKDAY_KEYS.map((key, index) => ({ day: index + 1, label: translate(`weekdaysShort.${key}`) })),
    labels: {
      rules: translate('rules'), rulesFor: translate('rulesFor'), addRule: translate('addRule'), rulesEmpty: translate('rulesEmpty'),
      startDate: translate('startDate'), endDate: translate('endDate'), weekdays: translate('weekdays'), adjustment: translate('adjustment'),
      amount: translate('amount'), priority: translate('priority'), removeRule: translate('removeRule'), save: translate('save'), close: translate('close'),
      cancel: translate('close'),
    },
  };
};

export type RateRulesView = ReturnType<typeof useRateRules>;
