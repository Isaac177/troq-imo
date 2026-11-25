import { useEffect, useMemo, useRef, useState, type ChangeEvent } from 'react';
import { useUiCopy } from '../copy/useUiCopy';
import type { CommandOption } from '../overlay/Command';
import { PHONE_COUNTRIES, countryFromDigits, detectCountryCode, digitsOf, findCountry, flagOf, nationalOf, toE164 } from './phoneCountries';

type Options = {
  value: string;
  onChange: (value: string) => void;
  fallbackCountry: string;
  detectCountry?: () => Promise<string | null>;
};

export const usePhoneField = ({ value, onChange, fallbackCountry, detectCountry }: Options) => {
  const { locale } = useUiCopy();
  const [open, setOpen] = useState(false);
  const digits = digitsOf(value);
  const inferredCountry = countryFromDigits(digits);
  const [selectedCode, setSelectedCode] = useState<string | null>(null);
  const selectedCountry = selectedCode ? findCountry(selectedCode) : null;
  const country = selectedCountry && selectedCountry.dial === inferredCountry?.dial ? selectedCountry : inferredCountry;
  const initial = useRef({ value, fallbackCountry, detectCountry });
  const currentValue = useRef(value);
  const change = useRef(onChange);
  const interacted = useRef(false);
  currentValue.current = value;
  change.current = onChange;

  useEffect(() => {
    if (initial.current.value) return;
    const fallbackCountry = findCountry(detectCountryCode(initial.current.fallbackCountry));
    const fallback = toE164(fallbackCountry.dial);
    if (!currentValue.current) {
      setSelectedCode(fallbackCountry.code);
      currentValue.current = fallback;
      change.current(fallback);
    }
    const detect = initial.current.detectCountry;
    if (!detect) return;
    let cancelled = false;
    void Promise.resolve().then(detect).then((code) => {
      if (cancelled || interacted.current || currentValue.current !== fallback || !code) return;
      const detectedCountry = findCountry(code);
      const detected = toE164(detectedCountry.dial);
      setSelectedCode(detectedCountry.code);
      currentValue.current = detected;
      change.current(detected);
    }).catch(() => undefined);
    return () => {
      cancelled = true;
    };
  }, []);

  const options = useMemo<CommandOption[]>(() => {
    let names: Intl.DisplayNames | null = null;
    try {
      names = new Intl.DisplayNames([locale], { type: 'region' });
    } catch {
      names = null;
    }
    return PHONE_COUNTRIES.map((item) => ({
      value: item.code,
      label: names?.of(item.code) ?? item.code,
      keywords: [item.code, `+${item.dial}`],
      detail: `+${item.dial}`,
      leading: flagOf(item.code),
    })).sort((first, second) => first.label.localeCompare(second.label, locale));
  }, [locale]);

  return {
    open,
    setOpen,
    options,
    text: toE164(digits),
    flag: country ? flagOf(country.code) : null,
    countryCode: country?.code ?? null,
    changeInput: (event: ChangeEvent<HTMLInputElement>) => {
      interacted.current = true;
      onChange(toE164(digitsOf(event.target.value)));
    },
    selectCountry: (code: string) => {
      interacted.current = true;
      const next = findCountry(code);
      setSelectedCode(next.code);
      onChange(toE164(`${next.dial}${country ? nationalOf(country, value) : digits}`));
      setOpen(false);
    },
  };
};
