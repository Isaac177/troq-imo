import { useEffect, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { Button, type TableColumn } from '@mungini/ui-web';
import { formatCdf, formatIsoDay } from '@mungini/utils';
import { createPromotion, listPlans, listPromotions, updatePromotion } from '@/services/rates/rates.api';
import type { Promotion, RatePlan } from '@/types/rates/rates';

const PAGE_SIZE = 20;
const KNOWN_ERRORS = ['FORBIDDEN', 'PERCENT_OUT_OF_RANGE', 'MIN_NIGHTS_OUT_OF_RANGE', 'REDEMPTION_LIMIT_OUT_OF_RANGE', 'PERIOD_INVALID', 'PROMOTION_LIMIT_BELOW_REDEEMED', 'PROMOTION_NOT_FOUND', 'RATE_PLAN_CHANGED'];

export const usePromotions = (propertyId: string, unitName: (id: string) => string, canWrite: boolean) => {
  const { t, i18n } = useTranslation('hostRates');
  const translate = t as unknown as (key: string) => string;
  const { client } = useSession();
  const [rows, setRows] = useState<Promotion[]>([]);
  const [total, setTotal] = useState(0);
  const [page, setPage] = useState(1);
  const [plans, setPlans] = useState<RatePlan[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [notice, setNotice] = useState<string | null>(null);
  const [createOpen, setCreateOpen] = useState(false);
  const [ratePlanId, setRatePlanId] = useState('all');
  const [minNights, setMinNights] = useState('1');
  const [discountPercent, setDiscountPercent] = useState('10.00');
  const [capAmount, setCapAmount] = useState('');
  const [startDate, setStartDate] = useState<string | null>(null);
  const [endDate, setEndDate] = useState<string | null>(null);
  const [priority, setPriority] = useState('0');
  const [redemptionLimit, setRedemptionLimit] = useState('');
  const [busy, setBusy] = useState(false);
  const [formError, setFormError] = useState<string | null>(null);

  const message = (code: string | null) => (code && KNOWN_ERRORS.includes(code) ? translate(`errors.${code}`) : translate('errors.generic'));

  useEffect(() => {
    setPage(1);
    setRows([]);
    setTotal(0);
  }, [propertyId]);

  useEffect(() => {
    if (!propertyId) return;
    let active = true;
    void listPlans(client, propertyId, 1, 100).then((result) => {
      if (active) setPlans(result.data?.items ?? []);
    });
    return () => {
      active = false;
    };
  }, [client, propertyId]);

  useEffect(() => {
    if (!propertyId) return;
    let active = true;
    setLoading(true);
    setError(null);
    void listPromotions(client, propertyId, page, PAGE_SIZE).then((result) => {
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
  }, [client, propertyId, page, translate]);

  const planName = (id: string | null | undefined) => {
    const plan = id ? plans.find((item) => item.id === id) : undefined;
    return plan ? unitName(plan.unitTypeId) || plan.unitTypeId : translate('allPlans');
  };
  const openCreate = () => {
    setRatePlanId('all');
    setMinNights('1');
    setDiscountPercent('10.00');
    setCapAmount('');
    setStartDate(null);
    setEndDate(null);
    setPriority('0');
    setRedemptionLimit('');
    setFormError(null);
    setCreateOpen(true);
  };

  const submit = async () => {
    if (!propertyId) return;
    setBusy(true);
    setFormError(null);
    const result = await createPromotion(client, propertyId, {
      ratePlanId: ratePlanId === 'all' ? null : ratePlanId,
      minNights: Number(minNights) || 1,
      discountPercent,
      capAmount: capAmount.trim() || null,
      startDate,
      endDate,
      priority: Number(priority) || 0,
      redemptionLimit: redemptionLimit.trim() ? Number(redemptionLimit) : null,
    });
    setBusy(false);
    if (result.error || !result.data) {
      setFormError(message(result.error));
      return;
    }
    setCreateOpen(false);
    setNotice(translate('promotionCreated'));
    setRows((current) => [result.data as Promotion, ...current]);
    setTotal((current) => current + 1);
  };

  const toggle = async (promotion: Promotion) => {
    setError(null);
    const status = promotion.status === 'ACTIVE' ? 'PAUSED' : 'ACTIVE';
    const result = await updatePromotion(client, propertyId, promotion.id, status);
    if (result.error || !result.data) {
      setError(message(result.error));
      return;
    }
    setRows((current) => current.map((row) => (row.id === result.data?.id ? (result.data as Promotion) : row)));
    setNotice(translate('promotionUpdated'));
  };

  const columns: TableColumn<Promotion>[] = [
    {
      key: 'discount',
      header: translate('discount'),
      cell: (row) => <div className="min-w-0"><p className="font-medium tabular-nums">{Number(row.discountPercent).toFixed(2)}%</p><p className="truncate text-label text-muted-foreground">{planName(row.ratePlanId)} · {translate('minNights')} {row.minNights}</p></div>,
    },
    {
      key: 'dates',
      header: translate('startDate'),
      hideBelow: 'sm',
      cell: (row) => <span className="whitespace-nowrap tabular-nums">{row.startDate ? formatIsoDay(row.startDate) : '—'} – {row.endDate ? formatIsoDay(row.endDate) : '—'}</span>,
    },
    { key: 'cap', header: translate('capAmount'), align: 'right', hideBelow: 'sm', cell: (row) => <span className="tabular-nums">{row.capAmount ? formatCdf(row.capAmount, i18n.language) : '—'}</span> },
    { key: 'redemptions', header: translate('redeemed'), align: 'right', hideBelow: 'sm', cell: (row) => <span className="tabular-nums">{row.redeemed}{row.redemptionLimit ? ` / ${row.redemptionLimit}` : ''}</span> },
    { key: 'status', header: translate('promotionStatus'), cell: (row) => translate(`promotionStatuses.${row.status}`) },
    { key: 'actions', header: '', align: 'right', cell: (row) => (canWrite ? <Button size="sm" variant="outline" onClick={() => void toggle(row)}>{row.status === 'ACTIVE' ? translate('pause') : translate('activate')}</Button> : null) },
  ];

  return {
    rows, total, page, setPage, pageSize: PAGE_SIZE, loading, error, notice, columns,
    createOpen, setCreateOpen, openCreate, busy, formError, submit,
    ratePlanId, setRatePlanId, minNights, setMinNights, discountPercent, setDiscountPercent,
    capAmount, setCapAmount, startDate, setStartDate, endDate, setEndDate, priority, setPriority,
    redemptionLimit, setRedemptionLimit,
    planOptions: [{ value: 'all', label: translate('allPlans') }, ...plans.map((plan) => ({ value: plan.id, label: `${unitName(plan.unitTypeId) || plan.unitTypeId} · v${plan.version}` }))],
    labels: {
      promotions: translate('promotions'), promotionsEmpty: translate('promotionsEmpty'), newPromotion: translate('newPromotion'),
      ratePlan: translate('ratePlan'), minNights: translate('minNights'), discount: translate('discount'), capAmount: translate('capAmount'),
      redemptions: translate('redemptions'), startDate: translate('startDate'), endDate: translate('endDate'), priority: translate('priority'),
      save: translate('save'), close: translate('close'), cancel: translate('close'),
    },
  };
};

export type PromotionsView = ReturnType<typeof usePromotions>;
