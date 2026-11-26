import Link from 'next/link';
import { cookies } from 'next/headers';
import { createServerI18n } from '@mungini/i18n/server';
import { formatCdf } from '@mungini/utils';
import SearchBar from '@/components/stays/SearchBar';
import { listStays, servedCities } from '@/services/guest/discovery.server';

type HomeProps = {
  searchParams: Promise<{ search?: string; city?: string; page?: string }>;
};

const pageNumber = (value: string | undefined) => {
  const parsed = Number(value);
  return Number.isFinite(parsed) && parsed > 0 ? Math.floor(parsed) : 1;
};

export default async function Page({ searchParams }: HomeProps) {
  const params = await searchParams;
  const city = params.city === 'KIN' ? 'KIN' : 'GOMA';
  const search = params.search ?? '';
  const page = pageNumber(params.page);
  const language = (await cookies()).get('mungini-language')?.value === 'fr' ? 'fr' : 'en';
  const i18n = createServerI18n(language);
  const t = i18n.getFixedT(language, 'guest') as unknown as (key: string) => string;
  const [cities, result] = await Promise.all([servedCities(), listStays(city, search, page, language)]);
  const items = result?.items ?? [];
  const total = result?.total ?? 0;
  const pageSize = result?.pageSize ?? 12;
  const pageCount = Math.max(1, Math.ceil(total / pageSize));

  return (
    <main className="mx-auto flex w-full max-w-6xl flex-col gap-6 px-3 py-6 md:px-6 md:py-10">
      <section className="flex flex-col gap-3">
        <h1 className="text-[22px] font-semibold md:text-[28px]">{t('homeTitle')}</h1>
        <p className="text-body text-muted-foreground">{t('homeSubtitle')}</p>
        <SearchBar cities={cities} city={city} search={search} />
      </section>

      {items.length === 0 ? (
        <p className="rounded-card border border-border bg-card px-4 py-10 text-center text-body text-muted-foreground">{t('noStays')}</p>
      ) : (
        <section className="grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3">
          {items.map((stay) => (
            <Link
              key={stay.id}
              href={`/stays/${stay.id}?city=${city}`}
              className="group flex flex-col overflow-hidden rounded-card border border-border bg-card transition-colors duration-200 ease-in-out hover:border-border hover:bg-muted/40"
            >
              {stay.cover?.url ? (
                <img src={stay.cover.url} alt={stay.cover.alt || stay.name} className="h-[160px] w-full object-cover" loading="lazy" />
              ) : (
                <div className="h-[160px] w-full bg-muted" aria-hidden="true" />
              )}
              <div className="flex flex-col gap-1 p-3">
                <p className="truncate text-body font-medium text-foreground">{stay.name}</p>
                <p className="text-label text-muted-foreground">
                  {stay.best.propertyType} · {stay.reviewCount} {t('reviews')}
                  {stay.verifiedRating ? ` · ${stay.verifiedRating.toFixed(1)}` : ''}
                </p>
                <p className="mt-1 text-body text-foreground">
                  {t('from')} <span className="font-semibold tabular-nums">{formatCdf(stay.best.nightly, language, 0)}</span> {t('perNight')}
                </p>
              </div>
            </Link>
          ))}
        </section>
      )}

      {pageCount > 1 ? (
        <nav className="flex items-center justify-center gap-2 text-body" aria-label={t('homeTitle')}>
          {page > 1 ? (
            <Link className="underline-offset-4 hover:underline" href={`/?${new URLSearchParams({ city, ...(search ? { search } : {}), page: String(page - 1) }).toString()}`}>
              {i18n.getFixedT(language, 'common')('previous')}
            </Link>
          ) : null}
          <span className="tabular-nums text-muted-foreground">
            {page} / {pageCount}
          </span>
          {page < pageCount ? (
            <Link className="underline-offset-4 hover:underline" href={`/?${new URLSearchParams({ city, ...(search ? { search } : {}), page: String(page + 1) }).toString()}`}>
              {i18n.getFixedT(language, 'common')('next')}
            </Link>
          ) : null}
        </nav>
      ) : null}
    </main>
  );
}
