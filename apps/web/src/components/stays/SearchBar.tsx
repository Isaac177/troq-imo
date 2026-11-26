'use client';

import { useState } from 'react';
import { useRouter } from 'next/navigation';
import { useTranslation } from '@mungini/i18n';
import { Button, Input, Select } from '@mungini/ui-web';

type SearchBarProps = {
  cities: { code: string; name: string }[];
  city: string;
  search: string;
};

export default function SearchBar({ cities, city, search }: SearchBarProps) {
  const router = useRouter();
  const { i18n } = useTranslation();
  const t = i18n.getFixedT(i18n.language, 'guest') as unknown as (key: string) => string;
  const [term, setTerm] = useState(search);
  const [selectedCity, setSelectedCity] = useState(city);

  const submit = (event: React.FormEvent) => {
    event.preventDefault();
    const query = new URLSearchParams({ city: selectedCity });
    if (term.trim()) query.set('search', term.trim());
    router.push(`/?${query.toString()}`);
  };

  return (
    <form onSubmit={submit} className="flex w-full flex-col gap-2 sm:flex-row">
      <div className="w-full sm:w-[150px]">
        <Select
          options={cities.map((item) => ({ value: item.code, label: item.name }))}
          value={selectedCity}
          onValueChange={setSelectedCity}
          ariaLabel={t('city')}
        />
      </div>
      <Input
        type="search"
        value={term}
        onChange={(event) => setTerm(event.target.value)}
        placeholder={t('searchPlaceholder')}
        aria-label={t('searchPlaceholder')}
      />
      <Button type="submit" variant="primary">
        {t('homeTitle')}
      </Button>
    </form>
  );
}
