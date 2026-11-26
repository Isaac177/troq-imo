import Link from 'next/link';
import { SaveFavorite } from '@/components/favorites/SaveFavorite';
import { BookingPanel } from '@/components/booking/BookingPanel';
import { cookies } from 'next/headers';
import { notFound } from 'next/navigation';
import { createServerI18n } from '@mungini/i18n/server';
import { getStay } from '@/services/guest/discovery.server';

type StayProps = {
  params: Promise<{ propertyId: string }>;
  searchParams: Promise<{ city?: string }>;
};

export default async function StayPage({ params, searchParams }: StayProps) {
  const { propertyId } = await params;
  const { city: requestedCity } = await searchParams;
  const city = requestedCity === 'KIN' ? 'KIN' : 'GOMA';
  const language = (await cookies()).get('mungini-language')?.value === 'fr' ? 'fr' : 'en';
  const i18n = createServerI18n(language);
  const t = i18n.getFixedT(language, 'guest') as unknown as (key: string) => string;
  const stay = await getStay(city, propertyId, language);

  if (!stay) notFound();

  return (
    <main className="mx-auto flex w-full max-w-5xl flex-col gap-5 px-3 py-6 md:px-6 md:py-10">
      <Link href={`/?city=${city}`} className="text-label text-muted-foreground underline-offset-4 hover:underline">
        {t('homeTitle')}
      </Link>
      <header className="flex flex-col gap-1">
        <h1 className="text-[22px] font-semibold md:text-[28px]">{stay.name}</h1>
        <p className="text-body text-muted-foreground">
          {stay.city}
          {stay.hostName ? ` · ${stay.hostName}` : ''}
        </p>
      </header>

      {stay.media.length > 0 ? (
        <section className="grid grid-cols-2 gap-2 md:grid-cols-4">
          {stay.media.slice(0, 4).map((media, index) => (
            <img
              key={media.id}
              src={media.url}
              alt={media.alt || stay.name}
              className={index === 0 ? 'col-span-2 h-[220px] w-full rounded-card object-cover md:col-span-2 md:h-[300px]' : 'h-[220px] w-full rounded-card object-cover md:h-[300px]'}
              loading={index === 0 ? 'eager' : 'lazy'}
            />
          ))}
        </section>
      ) : null}

      <SaveFavorite propertyId={propertyId} city={city} />
      <BookingPanel propertyId={propertyId} city={city} stays={stay.unitTypes.map((unit) => ({ id: unit.id, name: unit.name }))} />

      <section className="flex flex-col gap-2 rounded-card border border-border bg-card p-4">
        <h2 className="text-title font-semibold">{t('detailAbout')}</h2>
        <p className="whitespace-pre-line text-body text-muted-foreground">{stay.description}</p>
      </section>

      {stay.amenities.length > 0 ? (
        <section className="flex flex-col gap-2 rounded-card border border-border bg-card p-4">
          <h2 className="text-title font-semibold">{t('amenities')}</h2>
          <ul className="flex flex-wrap gap-2">
            {stay.amenities.map((amenity) => (
              <li key={amenity} className="rounded-chip bg-muted px-2 py-1 text-label text-muted-foreground">
                {amenity}
              </li>
            ))}
          </ul>
        </section>
      ) : null}

      {stay.unitTypes.length > 0 ? (
        <section className="flex flex-col gap-3">
          <h2 className="text-title font-semibold">{t('stays')}</h2>
          <ul className="grid grid-cols-1 gap-3 sm:grid-cols-2">
            {stay.unitTypes.map((unit) => (
              <li key={unit.id} className="flex flex-col gap-1 rounded-card border border-border bg-card p-3">
                <p className="text-body font-medium text-foreground">{unit.name}</p>
                <p className="text-label text-muted-foreground">{unit.description}</p>
                <p className="text-label text-muted-foreground tabular-nums">
                  {t('adults')}: {unit.maxAdults} · {t('children')}: {unit.maxChildren}
                </p>
              </li>
            ))}
          </ul>
        </section>
      ) : null}
    </main>
  );
}
