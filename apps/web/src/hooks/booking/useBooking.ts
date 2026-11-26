import { useEffect, useMemo, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { formatCdf } from '@mungini/utils';
import { createBooking, createQuote, getAvailability } from '@/services/booking/booking.api';
import type { AvailabilityOption, Policy, Quote, StayOption } from '@/types/booking/booking';

const parsePolicies = (value: unknown): Policy[] | null => {
  if (!Array.isArray(value) || value.length !== 2) return null;
  const policies = value as Policy[];
  if (!policies.every((p) => p && typeof p.id === 'string' && p.content && typeof p.content === 'object')) return null;
  const cancellation = policies.find((p) => p.kind === 'CANCELLATION');
  const payment = policies.find((p) => p.kind === 'PAYMENT');
  if (!cancellation || !payment || typeof cancellation.content.freeUntilHours !== 'number' || typeof cancellation.content.penaltyPercent !== 'number') return null;
  if (payment.content.collection !== 'PAY_AT_PROPERTY' && payment.content.collection !== 'PREPAY') return null;
  return policies;
};

export const useBooking = (propertyId: string, propertyCity: string, stays: StayOption[]) => {
  const { t, i18n } = useTranslation('booking');
  const { client, status, city, session, setCity } = useSession();
  const [range, setRange] = useState<{ start: string | null; end: string | null }>({ start: null, end: null });
  const [adults, setAdults] = useState('1');
  const [children, setChildren] = useState('0');
  const [options, setOptions] = useState<AvailabilityOption[] | null>(null);
  const [selected, setSelected] = useState('');
  const [quote, setQuote] = useState<Quote | null>(null);
  const [name, setName] = useState('');
  const [email, setEmail] = useState('');
  const [request, setRequest] = useState('');
  const [accepted, setAccepted] = useState(false);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState('');
  const [success, setSuccess] = useState(false);
  const [now, setNow] = useState(Date.now());
  const policies = useMemo(() => quote ? parsePolicies(quote.policies) : null, [quote]);
  const cancellation = policies?.find((p) => p.kind === 'CANCELLATION');
  const payment = policies?.find((p) => p.kind === 'PAYMENT');
  const expired = !!quote && (quote.expired || Date.parse(quote.expiresAt) <= now);
  const choice = options?.find((option) => option.ratePlanId === selected);
  const paymentSupported = payment?.content.collection === 'PAY_AT_PROPERTY';
  const mealName = (value: string) => ({ ROOM_ONLY: t('roomOnly'), BREAKFAST: t('breakfast'), HALF_BOARD: t('halfBoard'), FULL_BOARD: t('fullBoard') }[value] ?? t('otherMeal'));
  useEffect(() => {
    if (!session) return;
    setName([session.user.firstName, session.user.lastName].filter(Boolean).join(' '));
    setEmail(session.user.email ?? '');
  }, [session]);
  useEffect(() => { if (!quote) return; const id = setInterval(() => setNow(Date.now()), 1000); return () => clearInterval(id); }, [quote]);
  const reset = () => { setOptions(null); setSelected(''); setQuote(null); setAccepted(false); setSuccess(false); setError(''); };
  const updateRange = (value: typeof range) => { setRange(value); reset(); };
  const updateAdults = (value: string) => { setAdults(value); reset(); };
  const updateChildren = (value: string) => { setChildren(value); reset(); };
  const availability = async () => {
    const a = Number(adults); const c = Number(children);
    if (!range.start || !range.end || range.end <= range.start || !Number.isInteger(a) || a < 1 || !Number.isInteger(c) || c < 0) { setError(t('invalid')); return; }
    setBusy(true); setError(''); setQuote(null); setAccepted(false);
    try { const data = await getAvailability(client, propertyId, range.start, range.end, a, c); setOptions(data.options); setSelected(''); }
    catch { setError(t('error')); } finally { setBusy(false); }
  };
  const loadQuote = async () => {
    if (!choice || !range.start || !range.end) return;
    setBusy(true); setError(''); setAccepted(false); setQuote(null);
    try { const data = await createQuote(client, { propertyId, unitTypeId: choice.unitTypeId, ratePlanId: choice.ratePlanId, checkIn: range.start, checkOut: range.end, adultCount: Number(adults), childCount: Number(children) }); setNow(Date.now()); setQuote(data); }
    catch { setError(t('error')); } finally { setBusy(false); }
  };
  const book = async () => {
    if (!quote || !policies || !paymentSupported || !accepted || busy || expired) return;
    if (!name.trim() || !/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email.trim())) { setError(t('emailRequired')); return; }
    setBusy(true); setError('');
    try { await createBooking(client, { quoteId: quote.id, acceptedPolicyVersionIds: policies.map((p) => p.id), guest: { fullName: name.trim(), contactEmail: email.trim() }, specialRequest: request.trim() || null }); setSuccess(true); setQuote(null); }
    catch { setError(t('error')); } finally { setBusy(false); }
  };
  const switchCity = async () => { setBusy(true); try { await setCity(propertyCity); setError(''); } catch { setError(t('error')); } finally { setBusy(false); } };
  return { t, status, city, propertyCity, homeCity: session?.user.city, range, updateRange, adults, updateAdults, children, updateChildren, options, selected, setSelected: (id: string) => { setSelected(id); setQuote(null); setAccepted(false); }, quote, name, setName, email, setEmail, request, setRequest, accepted, setAccepted, busy, error, success, expired, policies, cancellation, payment, paymentSupported, availability, loadQuote, book, switchCity, optionsForSelect: (options ?? []).map((o) => ({ value: o.ratePlanId, label: `${stays.find((s) => s.id === o.unitTypeId)?.name ?? ''} · ${o.bookingMode === 'INSTANT' ? t('instant') : t('request')} · ${mealName(o.mealPlan)}${o.total ? ` · ${formatCdf(o.total, i18n.language)}` : ''}` })), money: (value: string) => formatCdf(value, i18n.language) };
};
