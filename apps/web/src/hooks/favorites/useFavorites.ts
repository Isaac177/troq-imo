import { useCallback, useEffect, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { createWishlist, listItems, listWishlists, removeItem } from '@/services/favorites/favorites.api';
import type { WishlistItemPage, WishlistPage } from '@/types/favorites/favorites';

export const useFavorites = () => {
  const { client, status, city, setCity } = useSession();
  const { i18n } = useTranslation();
  const t = i18n.getFixedT(i18n.language, 'guest') as unknown as (key: string) => string;
  const [cities, setCities] = useState<{ code: string; name: string }[]>([]);
  const [lists, setLists] = useState<WishlistPage | null>(null);
  const [items, setItems] = useState<WishlistItemPage | null>(null);
  const [selected, setSelected] = useState('');
  const [listPage, setListPage] = useState(1);
  const [itemPage, setItemPage] = useState(1);
  const [name, setName] = useState('');
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState(false);
  const [refresh, setRefresh] = useState(0);
  const [loading, setLoading] = useState(true);
  const [loadingItems, setLoadingItems] = useState(false);

  useEffect(() => {
    void client.api.GET('/api/public/cities').then(({ data }) => { if (data) setCities(data.data); }).catch(() => setError(true));
  }, [client]);

  useEffect(() => { setListPage(1); setItemPage(1); setSelected(''); }, [city]);

  useEffect(() => {
    if (status !== 'authenticated') return;
    let active = true;
    setLoading(true);
    void listWishlists(client, listPage).then((data) => {
      if (!active) return;
      setLists(data);
      setSelected((current) => data.items.some((list) => list.id === current) ? current : data.items[0]?.id ?? '');
      setError(false);
    }).catch(() => { if (active) setError(true); }).finally(() => { if (active) setLoading(false); });
    return () => { active = false; };
  }, [client, status, city, listPage, refresh]);

  useEffect(() => {
    setItems(null);
    if (!selected || status !== 'authenticated') return;
    let active = true;
    setLoadingItems(true);
    void listItems(client, selected, itemPage, i18n.language).then((data) => { if (active) setItems(data); }).catch(() => { if (active) setError(true); }).finally(() => { if (active) setLoadingItems(false); });
    return () => { active = false; };
  }, [client, status, selected, itemPage, i18n.language, refresh]);

  const create = useCallback(async () => {
    if (!name.trim() || busy) return;
    setBusy(true);
    try { await createWishlist(client, name.trim()); setName(''); setListPage(1); setRefresh((v) => v + 1); setError(false); }
    catch { setError(true); } finally { setBusy(false); }
  }, [client, name, busy]);

  const remove = useCallback(async (propertyId: string) => {
    if (!selected || busy) return;
    setBusy(true);
    try { await removeItem(client, selected, propertyId); setItemPage(1); setRefresh((v) => v + 1); setError(false); }
    catch { setError(true); } finally { setBusy(false); }
  }, [client, selected, busy]);

  return { status, city, cities, changeCity: (value: string) => { void setCity(value).catch(() => setError(true)); }, lists, items, selected, loading, loadingItems, error, busy, name, setName, create, remove, listPage, itemPage,
    select: (id: string) => { setSelected(id); setItemPage(1); },
    previousList: () => setListPage((v) => Math.max(1, v - 1)), nextList: () => setListPage((v) => v + 1),
    previousItem: () => setItemPage((v) => Math.max(1, v - 1)), nextItem: () => setItemPage((v) => v + 1),
    labels: { title: t('favoritesTitle'), signIn: t('signIn'), signInSubtitle: t('signInSubtitle'), name: t('favoritesName'), create: t('favoritesCreate'), empty: t('favoritesEmpty'), noItems: t('favoritesNoItems'), remove: t('favoritesRemove'), error: t('errorGeneric'), loading: t('favoritesLoading'), previous: t('invitationPrevious'), next: t('invitationNext'), city: t('city'), page: t('invitationPage') },
  };
};
