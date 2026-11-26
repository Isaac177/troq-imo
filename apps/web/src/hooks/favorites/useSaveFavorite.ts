import { useEffect, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { addItem, listItems, listWishlists, removeItem } from '@/services/favorites/favorites.api';
import type { Wishlist } from '@/types/favorites/favorites';

export const useSaveFavorite = (propertyId: string, propertyCity: string) => {
  const { client, status, city, setCity } = useSession();
  const { i18n } = useTranslation();
  const t = i18n.getFixedT(i18n.language, 'guest') as unknown as (key: string) => string;
  const [lists, setLists] = useState<Wishlist[]>([]);
  const [selected, setSelected] = useState('');
  const [saved, setSaved] = useState(false);
  const [loadingLists, setLoadingLists] = useState(false);
  const [loadingItem, setLoadingItem] = useState(false);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState(false);
  const [refresh, setRefresh] = useState(0);

  useEffect(() => {
    if (status !== 'authenticated' || city !== propertyCity) return;
    let active = true;
    setLoadingLists(true);
    void listWishlists(client, 1, 20).then((data) => {
      if (!active) return;
      setLists(data.items);
      setSelected((current) => data.items.some((list) => list.id === current) ? current : data.items[0]?.id ?? '');
      setError(false);
    }).catch(() => { if (active) setError(true); }).finally(() => { if (active) setLoadingLists(false); });
    return () => { active = false; };
  }, [client, status, city, propertyCity, refresh]);

  useEffect(() => {
    setSaved(false);
    if (!selected || status !== 'authenticated' || city !== propertyCity) return;
    let active = true;
    setLoadingItem(true);
    void listItems(client, selected, 1, i18n.language, propertyId).then((data) => { if (active) setSaved(data.items.length > 0); })
      .catch(() => { if (active) setError(true); }).finally(() => { if (active) setLoadingItem(false); });
    return () => { active = false; };
  }, [client, status, selected, city, propertyCity, propertyId, i18n.language]);

  const loading = loadingLists || loadingItem;
  const toggle = async () => {
    if (!selected || busy || loading) return;
    setBusy(true);
    try {
      if (saved) await removeItem(client, selected, propertyId);
      else await addItem(client, selected, propertyId);
      setSaved(!saved);
      setRefresh((v) => v + 1);
      setError(false);
    } catch { setError(true); } finally { setBusy(false); }
  };
  const switchCity = async () => {
    setBusy(true);
    try { await setCity(propertyCity); setError(false); }
    catch { setError(true); } finally { setBusy(false); }
  };

  return { status, city, propertyCity, lists, selected, setSelected, saved, loading, busy, error, toggle, switchCity,
    labels: { title: t('favoritesTitle'), choose: t('favoritesChoose'), save: t('favoritesSave'), remove: t('favoritesRemove'), empty: t('favoritesEmpty'), manage: t('favoritesManage'), signIn: t('signIn'), switchCity: t('favoritesSwitchCity'), error: t('errorGeneric'), loading: t('favoritesLoading') },
  };
};
