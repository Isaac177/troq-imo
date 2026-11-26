import { useEffect, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { archiveUnitType, createUnitType, listCategories, listUnitTypes, updateUnitType } from '@/services/properties/properties.api';
import type { Category, UnitType } from '@/services/properties/properties.api';

const KNOWN_ERRORS = ['UNIT_TYPE_NOT_FOUND', 'UNIT_TYPE_NAME_INVALID', 'UNIT_TYPE_CAPACITY_INVALID', 'UNIT_TYPE_QUANTITY_INVALID', 'UNIT_TYPE_MODEL_INVALID', 'UNIT_TYPE_CATEGORY_UNKNOWN', 'UNIT_TYPE_VERSION_CONFLICT', 'UNIT_TYPE_QUANTITY_LOCKED', 'UNIT_TYPE_HAS_RESERVATIONS', 'FORBIDDEN'];

export const usePropertyUnits = (propertyId: string | null, onChanged: () => void) => {
  const { t, i18n } = useTranslation('host');
  const translate = t as unknown as (key: string) => string;
  const { client } = useSession();
  const [units, setUnits] = useState<UnitType[]>([]);
  const [categories, setCategories] = useState<Category[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [notice, setNotice] = useState<string | null>(null);
  const [open, setOpen] = useState(false);
  const [target, setTarget] = useState<UnitType | null>(null);
  const [nameEn, setNameEn] = useState('');
  const [nameFr, setNameFr] = useState('');
  const [categoryCode, setCategoryCode] = useState('');
  const [inventoryModel, setInventoryModel] = useState('WHOLE_UNIT');
  const [maxAdults, setMaxAdults] = useState('2');
  const [maxChildren, setMaxChildren] = useState('0');
  const [bedrooms, setBedrooms] = useState('1');
  const [bathrooms, setBathrooms] = useState('1');
  const [beds, setBeds] = useState('1');
  const [quantity, setQuantity] = useState('1');
  const [descriptionEn, setDescriptionEn] = useState('');
  const [descriptionFr, setDescriptionFr] = useState('');
  const [busy, setBusy] = useState(false);
  const [formError, setFormError] = useState<string | null>(null);
  const [archiveTarget, setArchiveTarget] = useState<UnitType | null>(null);
  const french = i18n.language.startsWith('fr');

  const message = (code: string | null) => (code && KNOWN_ERRORS.includes(code) ? translate(`unitErrors.${code}`) : translate('unitErrors.generic'));

  useEffect(() => {
    let active = true;
    void listCategories(client).then((result) => {
      if (active) setCategories(result.data ?? []);
    });
    return () => {
      active = false;
    };
  }, [client]);

  useEffect(() => {
    setUnits([]);
    setNotice(null);
    if (!propertyId) return;
    let active = true;
    setLoading(true);
    setError(null);
    void listUnitTypes(client, propertyId).then((result) => {
      if (!active) return;
      setLoading(false);
      if (result.error) {
        setError(translate('unitErrors.generic'));
        return;
      }
      setUnits(result.data ?? []);
    });
    return () => {
      active = false;
    };
  }, [client, propertyId, translate]);

  const reset = () => {
    setNameEn('');
    setNameFr('');
    setCategoryCode(categories[0]?.code ?? '');
    setInventoryModel('WHOLE_UNIT');
    setMaxAdults('2');
    setMaxChildren('0');
    setBedrooms('1');
    setBathrooms('1');
    setBeds('1');
    setQuantity('1');
    setDescriptionEn('');
    setDescriptionFr('');
    setFormError(null);
  };

  const openCreate = () => {
    setTarget(null);
    reset();
    setOpen(true);
  };

  const openEdit = (unit: UnitType) => {
    setTarget(unit);
    setNameEn(unit.nameEn);
    setNameFr(unit.nameFr);
    setCategoryCode(unit.categoryCode);
    setInventoryModel(unit.inventoryModel);
    setMaxAdults(String(unit.maxAdults));
    setMaxChildren(String(unit.maxChildren));
    setBedrooms(String(unit.bedroomCount));
    setBathrooms(String(unit.bathroomCount));
    setBeds(String(unit.bedCount));
    setQuantity(String(unit.sellableQuantity));
    setDescriptionEn(unit.descriptionEn);
    setDescriptionFr(unit.descriptionFr);
    setFormError(null);
    setOpen(true);
  };

  const submit = async () => {
    if (!propertyId) return;
    setBusy(true);
    setFormError(null);
    const body = {
      nameEn, nameFr, categoryCode, inventoryModel,
      maxAdults: Number(maxAdults) || 0, maxChildren: Number(maxChildren) || 0,
      bedroomCount: Number(bedrooms) || 0, bathroomCount: Number(bathrooms) || 0, bedCount: Number(beds) || 0,
      sellableQuantity: Number(quantity) || 1, descriptionEn, descriptionFr,
    };
    const result = target
      ? await updateUnitType(client, propertyId, target.id, { ...body, expectedVersion: target.version })
      : await createUnitType(client, propertyId, body);
    setBusy(false);
    if (result.error || !result.data) {
      setFormError(message(result.error));
      return;
    }
    setOpen(false);
    setNotice(target ? translate('unitSaved') : translate('unitCreated'));
    onChanged();
    const refreshed = await listUnitTypes(client, propertyId);
    setUnits(refreshed.data ?? []);
  };

  const confirmArchive = async () => {
    if (!propertyId || !archiveTarget) return;
    setBusy(true);
    const result = await archiveUnitType(client, propertyId, archiveTarget.id, archiveTarget.version);
    setBusy(false);
    if (result.error) {
      setError(message(result.error));
      setArchiveTarget(null);
      return;
    }
    setUnits((current) => current.filter((unit) => unit.id !== archiveTarget.id));
    setArchiveTarget(null);
    setNotice(translate('unitArchived'));
    onChanged();
  };

  return {
    units, categories, loading, error, notice,
    open, setOpen, target, openCreate, openEdit, nameEn, setNameEn, nameFr, setNameFr,
    categoryCode, setCategoryCode, inventoryModel, setInventoryModel,
    maxAdults, setMaxAdults, maxChildren, setMaxChildren, bedrooms, setBedrooms,
    bathrooms, setBathrooms, beds, setBeds, quantity, setQuantity,
    descriptionEn, setDescriptionEn, descriptionFr, setDescriptionFr,
    busy, formError, submit, archiveTarget, setArchiveTarget, confirmArchive,
    unitName: (unit: UnitType) => (french ? unit.nameFr : unit.nameEn) || unit.nameEn || unit.nameFr,
    categoryName: (unit: UnitType) => {
      const category = categories.find((item) => item.code === unit.categoryCode);
      return category ? (french ? category.nameFr : category.nameEn) || category.nameEn : unit.categoryCode;
    },
    categoryOptions: categories.map((category) => ({ value: category.code, label: (french ? category.nameFr : category.nameEn) || category.nameEn })),
    modelOptions: [
      { value: 'WHOLE_UNIT', label: translate('unitModels.WHOLE_UNIT') },
      { value: 'POOLED_ROOM', label: translate('unitModels.POOLED_ROOM') },
    ],
    labels: {
      unitsTitle: translate('propertyUnits'), addUnit: translate('addUnit'), editUnit: translate('editUnit'), unitNameEn: translate('unitNameEn'),
      unitNameFr: translate('unitNameFr'), unitCategory: translate('unitCategory'), unitModel: translate('unitModel'),
      unitMaxAdults: translate('unitMaxAdults'), unitMaxChildren: translate('unitMaxChildren'), unitBedrooms: translate('unitBedrooms'),
      unitBathrooms: translate('unitBathrooms'), unitBeds: translate('unitBeds'), unitQuantity: translate('unitQuantity'),
      unitDescriptionEn: translate('unitDescriptionEn'), unitDescriptionFr: translate('unitDescriptionFr'), unitSave: translate('unitSave'),
      unitArchive: translate('unitArchive'), unitNoUnits: translate('unitNoUnits'), close: translate('close'), cancel: translate('close'),
      confirmArchiveTitle: translate('confirmArchiveUnitTitle'), confirmArchiveBody: translate('confirmArchiveUnitBody'),
      error: translate('unitErrors.generic'),
    },
  };
};

export type PropertyUnitsView = ReturnType<typeof usePropertyUnits>;
