import { useEffect, useRef, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { formatCdf, formatIsoDay } from '@mungini/utils';
import { previewPrice } from '@/services/rates/rates.api';
import type { PricePreview, RatePlan } from '@/types/rates/rates';

export const usePricePreview = (propertyId: string, plan: RatePlan) => {
  const { t, i18n } = useTranslation('hostRates');
  const translate = t as unknown as (key: string) => string;
  const { client } = useSession();
  const [checkIn, setCheckIn] = useState<string | null>(null);
  const [checkOut, setCheckOut] = useState<string | null>(null);
  const [adults, setAdults] = useState('2');
  const [children, setChildren] = useState('0');
  const [preview, setPreview] = useState<PricePreview | null>(null);
  const [error, setError] = useState('');
  const [busy, setBusy] = useState(false);
  const requestId = useRef(0);

  useEffect(() => () => { requestId.current += 1; }, []);

  useEffect(() => {
    requestId.current += 1;
    setPreview(null);
    setError('');
    setBusy(false);
  }, [propertyId, plan.id, checkIn, checkOut, adults, children]);

  const submit = async () => {
    const adultCount = Number(adults);
    const childCount = Number(children);
    if (!checkIn || !checkOut || checkIn >= checkOut || !/^\d+$/.test(adults) || !/^\d+$/.test(children) || !Number.isSafeInteger(adultCount) || !Number.isSafeInteger(childCount) || adultCount < 1 || adultCount > 2147483647 || childCount > 2147483647) {
      setError(translate('previewInvalid'));
      return;
    }
    const current = ++requestId.current;
    setBusy(true);
    setError('');
    try {
      const result = await previewPrice(client, { propertyId, unitTypeId: plan.unitTypeId, ratePlanId: plan.id, checkIn, checkOut, adultCount, childCount });
      if (current !== requestId.current) return;
      if (!result.data || result.error) {
        setPreview(null);
        setError(translate('previewError'));
      } else {
        setPreview(result.data);
      }
    } catch {
      if (current === requestId.current) setError(translate('previewError'));
    } finally {
      if (current === requestId.current) setBusy(false);
    }
  };

  const ruleOutcome = (status: string, reason?: string | null) => {
    if (status === 'APPLIED') return translate('ruleApplied');
    if (reason === 'OUTSIDE_DATE_RANGE') return translate('ruleOutsideDates');
    if (reason === 'WEEKDAY_NOT_SELECTED') return translate('ruleWrongWeekday');
    if (reason === 'SUPERSEDED_BY_HIGHER_PRIORITY_BASE') return translate('ruleSuperseded');
    return translate('ruleExcluded');
  };
  const money = (value: string) => formatCdf(value, i18n.language);
  const nights = preview?.nights.map((night) => ({
    date: night.explanation.date,
    dateLabel: formatIsoDay(night.explanation.date),
    amount: money(night.amount),
    rules: night.explanation.rules.map((rule) => ({
      id: rule.ruleId,
      label: translate(`adjustments.${rule.adjustment}`),
      amount: rule.adjustment === 'PERCENTAGE' ? `${rule.amount}%` : money(rule.amount),
      outcome: ruleOutcome(rule.status, rule.reason),
    })),
  })) ?? [];

  return {
    checkIn, setCheckIn, checkOut, setCheckOut, adults, setAdults, children, setChildren, busy, error, submit,
    result: preview ? { subtotal: money(preview.subtotal), discount: money(preview.discount?.amount ?? '0'), total: money(preview.total), nights } : null,
    labels: {
      title: translate('preview'), hint: translate('previewHint'), checkIn: translate('checkIn'), checkOut: translate('checkOut'),
      adults: translate('adults'), children: translate('children'), run: translate('previewRun'), subtotal: translate('subtotal'),
      discount: translate('discountAmount'), total: translate('total'), breakdown: translate('nightlyBreakdown'),
    },
  };
};
