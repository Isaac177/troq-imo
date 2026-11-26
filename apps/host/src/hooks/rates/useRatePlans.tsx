import { useEffect, useMemo, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import type { TableColumn } from '@mungini/ui-web';
import { Button } from '@mungini/ui-web';
import { formatCdf } from '@mungini/utils';
import { hostApi } from '@/services/host/host.api';
import { createPlan, listPlans, listPolicies, listUnitTypes, updatePlan } from '@/services/rates/rates.api';
import type { Policy, PropertyOption, RatePlan, UnitType } from '@/types/rates/rates';

const PAGE_SIZE = 20;
const KNOWN_ERRORS = ['FORBIDDEN', 'CHOICE_INVALID', 'PERCENT_OUT_OF_RANGE', 'BASE_RATE_NOT_POSITIVE', 'BASE_RATE_OVERLAP', 'NO_BASE_RATE', 'RATE_PLAN_CHANGED', 'RATE_PLAN_NOT_FOUND', 'PERIOD_INVALID'];

export const useRatePlans = () => {
  const { t, i18n } = useTranslation('hostRates');
  const { t: th } = useTranslation('host');
  const translate = t as unknown as (key: string) => string;
  const translateHost = th as unknown as (key: string) => string;
  const { session, client, can } = useSession();
  const hosts = session?.hosts ?? [];
  const [hostId, setHostId] = useState(hosts[0]?.hostId ?? '');
  const [properties, setProperties] = useState<PropertyOption[]>([]);
  const [propertyId, setPropertyId] = useState('');
  const [rows, setRows] = useState<RatePlan[]>([]);
  const [total, setTotal] = useState(0);
  const [page, setPage] = useState(1);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [notice, setNotice] = useState<string | null>(null);
  const [unitTypes, setUnitTypes] = useState<UnitType[]>([]);
  const [cancelPolicies, setCancelPolicies] = useState<Policy[]>([]);
  const [payPolicies, setPayPolicies] = useState<Policy[]>([]);
  const [selectedPlan, setSelectedPlan] = useState<RatePlan | null>(null);
  const [createOpen, setCreateOpen] = useState(false);
  const [unitTypeId, setUnitTypeId] = useState('');
  const [occupancyBasis, setOccupancyBasis] = useState('PER_UNIT');
  const [mealPlan, setMealPlan] = useState('ROOM_ONLY');
  const [bookingMode, setBookingMode] = useState('INSTANT');
  const [cancelPolicyId, setCancelPolicyId] = useState('');
  const [payPolicyId, setPayPolicyId] = useState('');
  const [childAmount, setChildAmount] = useState('0.00');
  const [busy, setBusy] = useState(false);
  const [formError, setFormError] = useState<string | null>(null);
  const [editTarget, setEditTarget] = useState<RatePlan | null>(null);
  const [editStatus, setEditStatus] = useState('DRAFT');
  const [editBookingMode, setEditBookingMode] = useState('INSTANT');
  const [editMealPlan, setEditMealPlan] = useState('ROOM_ONLY');
  const [editChildAmount, setEditChildAmount] = useState('0.00');
  const french = i18n.language.startsWith('fr');

  const message = (code: string | null) => (code && KNOWN_ERRORS.includes(code) ? translate(`errors.${code}`) : translate('errors.generic'));

  useEffect(() => {
    if (!hosts.some((host) => host.hostId === hostId)) setHostId(hosts[0]?.hostId ?? '');
  }, [hosts, hostId]);

  useEffect(() => {
    if (!hostId) {
      setProperties([]);
      setPropertyId('');
      return;
    }
    let active = true;
    void hostApi(client).properties(hostId, 1, 100, '').then((result) => {
      if (!active || result.error) return;
      const options = (result.data?.items ?? []).map((property) => ({ value: property.id, label: (french ? property.nameFr : property.nameEn) || property.nameEn || property.nameFr }));
      setProperties(options);
      setPropertyId((current) => (options.some((option) => option.value === current) ? current : options[0]?.value ?? ''));
    });
    return () => {
      active = false;
    };
  }, [client, hostId, french]);

  useEffect(() => {
    setPage(1);
    setSelectedPlan(null);
  }, [propertyId]);

  useEffect(() => {
    if (!propertyId) return;
    let active = true;
    void Promise.all([listUnitTypes(client, propertyId), listPolicies(client, 'CANCELLATION'), listPolicies(client, 'PAYMENT')]).then(([units, cancellation, payment]) => {
      if (!active) return;
      setUnitTypes(units.data?.items ?? []);
      setCancelPolicies(cancellation.data ?? []);
      setPayPolicies(payment.data ?? []);
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
    void listPlans(client, propertyId, page, PAGE_SIZE).then((result) => {
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

  const unitName = (id: string) => {
    const unit = unitTypes.find((item) => item.id === id);
    if (!unit) return '';
    return (french ? unit.nameFr : unit.nameEn) || unit.nameEn || unit.nameFr;
  };
  const policyLabel = (policy: Policy) => {
    const content = policy.content as Record<string, unknown>;
    if (policy.kind === 'CANCELLATION') return `v${policy.versionNumber} · ${content.freeUntilHours ?? 0}h · ${content.penaltyPercent ?? 0}%`;
    const collection = typeof content.collection === 'string' ? content.collection : '';
    return `v${policy.versionNumber} · ${collection === 'PREPAY' || collection === 'PAY_AT_PROPERTY' ? translate(`collections.${collection}`) : collection}`;
  };

  const openCreate = () => {
    setUnitTypeId(unitTypes[0]?.id ?? '');
    setOccupancyBasis('PER_UNIT');
    setMealPlan('ROOM_ONLY');
    setBookingMode('INSTANT');
    setCancelPolicyId(cancelPolicies[0]?.id ?? '');
    setPayPolicyId(payPolicies[0]?.id ?? '');
    setChildAmount('0.00');
    setFormError(null);
    setCreateOpen(true);
  };

  const submitCreate = async () => {
    if (!propertyId || !unitTypeId || !cancelPolicyId || !payPolicyId) return;
    setBusy(true);
    setFormError(null);
    const result = await createPlan(client, propertyId, { unitTypeId, occupancyBasis, mealPlan, bookingMode, cancellationPolicyVersionId: cancelPolicyId, paymentPolicyVersionId: payPolicyId, childNightlyAmount: childAmount });
    setBusy(false);
    if (result.error || !result.data) {
      setFormError(message(result.error));
      return;
    }
    setCreateOpen(false);
    setNotice(translate('created'));
    setPage(1);
    setSelectedPlan(result.data);
    const refreshed = await listPlans(client, propertyId, 1, PAGE_SIZE);
    setRows(refreshed.data?.items ?? []);
    setTotal(refreshed.data?.total ?? 0);
  };

  const openEdit = (plan: RatePlan) => {
    setEditTarget(plan);
    setEditStatus(plan.status);
    setEditBookingMode(plan.bookingMode);
    setEditMealPlan(plan.mealPlan);
    setEditChildAmount(plan.childNightlyAmount);
    setFormError(null);
  };

  const submitEdit = async () => {
    if (!propertyId || !editTarget) return;
    setBusy(true);
    setFormError(null);
    const result = await updatePlan(client, propertyId, editTarget.id, { expectedVersion: editTarget.version, status: editStatus, bookingMode: editBookingMode, mealPlan: editMealPlan, childNightlyAmount: editChildAmount });
    setBusy(false);
    if (result.error || !result.data) {
      setFormError(message(result.error));
      return;
    }
    setEditTarget(null);
    setNotice(translate('created'));
    setRows((current) => current.map((plan) => (plan.id === result.data?.id ? result.data : plan)));
    setSelectedPlan((current) => (current?.id === result.data?.id ? result.data : current));
  };

  const columns: TableColumn<RatePlan>[] = [
    { key: 'unit', header: translate('unitType'), cell: (row) => <div className="min-w-0"><p className="truncate font-medium">{unitName(row.unitTypeId) || row.unitTypeId}</p><p className="text-label text-muted-foreground">{translate(`occupancies.${row.occupancyBasis}`)}</p></div> },
    { key: 'mode', header: translate('bookingMode'), cell: (row) => translate(`bookingModes.${row.bookingMode}`), hideBelow: 'sm' },
    { key: 'meal', header: translate('mealPlan'), cell: (row) => translate(`meals.${row.mealPlan}`), hideBelow: 'sm' },
    { key: 'status', header: translate('status'), cell: (row) => translate(`planStatuses.${row.status}`) },
    { key: 'child', header: translate('childAmount'), align: 'right', hideBelow: 'sm', cell: (row) => <span className="tabular-nums">{formatCdf(row.childNightlyAmount, i18n.language)}</span> },
    {
      key: 'actions',
      header: '',
      align: 'right',
      cell: (row) => (
        <div className="flex justify-end gap-1">
          <Button size="sm" variant="link" onClick={() => setSelectedPlan(row)}>{translate('rules')}</Button>
          <Button size="sm" variant="link" onClick={() => openEdit(row)}>{translate('editPlan')}</Button>
        </div>
      ),
    },
  ];

  return {
    hosts, hasHost: hosts.length > 0, hostId, setHostId,
    properties, propertyId, setPropertyId,
    rows, total, page, setPage, pageSize: PAGE_SIZE, loading, error, notice, columns,
    unitName,
    canWrite: can('pricing.write'),
    canPromote: can('pricing.campaigns'),
    selectedPlan, setSelectedPlan,
    createOpen, setCreateOpen, unitTypes, unitTypeId, setUnitTypeId,
    occupancyBasis, setOccupancyBasis, mealPlan, setMealPlan, bookingMode, setBookingMode,
    cancelPolicies, payPolicies, cancelPolicyId, setCancelPolicyId, payPolicyId, setPayPolicyId,
    childAmount, setChildAmount, busy, formError, submitCreate, openCreate,
    editTarget, setEditTarget, editStatus, setEditStatus, editBookingMode, setEditBookingMode, editMealPlan, setEditMealPlan, editChildAmount, setEditChildAmount, submitEdit,
    unitOptions: unitTypes.map((unit) => ({ value: unit.id, label: (french ? unit.nameFr : unit.nameEn) || unit.nameEn || unit.nameFr })),
    cancelOptions: cancelPolicies.map((policy) => ({ value: policy.id, label: policyLabel(policy) })),
    payOptions: payPolicies.map((policy) => ({ value: policy.id, label: policyLabel(policy) })),
    occupancyOptions: ['PER_UNIT', 'PER_GUEST'].map((value) => ({ value, label: translate(`occupancies.${value}`) })),
    mealOptions: ['ROOM_ONLY', 'BREAKFAST', 'HALF_BOARD', 'FULL_BOARD'].map((value) => ({ value, label: translate(`meals.${value}`) })),
    bookingOptions: ['INSTANT', 'REQUEST'].map((value) => ({ value, label: translate(`bookingModes.${value}`) })),
    statusOptions: ['DRAFT', 'ACTIVE', 'PAUSED', 'ARCHIVED'].map((value) => ({ value, label: translate(`planStatuses.${value}`) })),
    labels: {
      title: translate('title'), subtitle: translate('subtitle'), property: translate('property'), chooseProperty: translate('chooseProperty'),
      plans: translate('plans'), plansEmpty: translate('plansEmpty'), newPlan: translate('newPlan'), editPlan: translate('editPlan'),
      unitType: translate('unitType'), occupancyBasis: translate('occupancyBasis'), mealPlan: translate('mealPlan'), bookingMode: translate('bookingMode'),
      status: translate('status'), childAmount: translate('childAmount'), cancellationPolicy: translate('cancellationPolicy'), paymentPolicy: translate('paymentPolicy'),
      rules: translate('rules'), save: translate('save'), close: translate('close'), available: translate('created'),
      noHost: translateHost('noHost'), noHostAction: translateHost('noHostAction'),
    },
  };
};

export type RatePlansView = ReturnType<typeof useRatePlans>;
