import { useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { createTemplate, updateTemplate } from '@/services/templates/templates.api';
import type { RequestResult, Template, TemplateCategory, TemplateInput } from '@/types/templates/templates';

const TEMPLATE_ACCESSIBILITY = ['STEP_FREE_ENTRY', 'GROUND_FLOOR', 'ACCESSIBLE_BATHROOM', 'WIDE_DOORWAYS', 'HEARING_LOOP'];
const KNOWN_ERRORS = [
  'FORBIDDEN',
  'HOST_NOT_MEMBER',
  'HOST_NOT_FOUND',
  'HOST_NOT_ACTIVE',
  'TEMPLATE_NOT_FOUND',
  'TEMPLATE_AMOUNT_INVALID',
  'TEMPLATE_STATUS_INVALID',
  'TEMPLATE_VERSION_CONFLICT',
  'UNIT_TYPE_NAME_INVALID',
  'UNIT_TYPE_CAPACITY_INVALID',
  'UNIT_TYPE_ACCESSIBILITY_INVALID',
  'UNIT_TYPE_MODEL_INVALID',
  'UNIT_TYPE_CATEGORY_UNKNOWN',
  'CATEGORY_UNKNOWN',
  'CATEGORY_NOT_FOUND',
  'FIELD_REQUIRED',
  'FIELD_TOO_LONG',
];

const whole = (value: string) => Math.max(0, Math.round(Number(value) || 0));

type Options = { hostId: string; categories: TemplateCategory[]; onSaved: () => void };

export const useTemplateEditor = ({ hostId, categories, onSaved }: Options) => {
  const { t, i18n } = useTranslation('hostTemplates');
  const translate = t as unknown as (key: string, options?: Record<string, unknown>) => string;
  const { client } = useSession();
  const french = i18n.language.startsWith('fr');
  const [open, setOpenState] = useState(false);
  const [target, setTarget] = useState<Template | null>(null);
  const [nameEn, setNameEn] = useState('');
  const [nameFr, setNameFr] = useState('');
  const [descriptionEn, setDescriptionEn] = useState('');
  const [descriptionFr, setDescriptionFr] = useState('');
  const [categoryCode, setCategoryCode] = useState('');
  const [inventoryModel, setInventoryModel] = useState('WHOLE_UNIT');
  const [maxAdults, setMaxAdults] = useState('2');
  const [maxChildren, setMaxChildren] = useState('0');
  const [bedrooms, setBedrooms] = useState('1');
  const [beds, setBeds] = useState('1');
  const [bathrooms, setBathrooms] = useState('1');
  const [baseNightlyAmount, setBaseNightlyAmount] = useState('0.00');
  const [status, setStatus] = useState('ACTIVE');
  const [accessibility, setAccessibility] = useState<string[]>([]);
  const [busy, setBusy] = useState(false);
  const [formError, setFormError] = useState<string | null>(null);

  const message = (code: string | null) => (code && KNOWN_ERRORS.includes(code) ? translate(`errors.${code}`) : translate('errors.generic'));

  const setOpen = (value: boolean) => {
    setOpenState(value);
    if (!value) setFormError(null);
  };

  const applyTemplate = (template: Template) => {
    setTarget(template);
    setNameEn(template.nameEn);
    setNameFr(template.nameFr);
    setDescriptionEn(template.descriptionEn);
    setDescriptionFr(template.descriptionFr);
    setCategoryCode(template.categoryCode);
    setInventoryModel(template.inventoryModel);
    setMaxAdults(String(template.maxAdults));
    setMaxChildren(String(template.maxChildren));
    setBedrooms(String(template.bedroomCount));
    setBeds(String(template.bedCount));
    setBathrooms(String(template.bathroomCount));
    setBaseNightlyAmount(template.baseNightlyAmount);
    setStatus(template.status);
    setAccessibility(template.accessibility);
    setFormError(null);
  };

  const openCreate = () => {
    setTarget(null);
    setNameEn('');
    setNameFr('');
    setDescriptionEn('');
    setDescriptionFr('');
    setCategoryCode(categories[0]?.code ?? '');
    setInventoryModel('WHOLE_UNIT');
    setMaxAdults('2');
    setMaxChildren('0');
    setBedrooms('1');
    setBeds('1');
    setBathrooms('1');
    setBaseNightlyAmount('0.00');
    setStatus('ACTIVE');
    setAccessibility([]);
    setFormError(null);
    setOpenState(true);
  };

  const openEdit = (template: Template) => {
    applyTemplate(template);
    setOpenState(true);
  };

  const toggleAccessibility = (code: string) => {
    setAccessibility((current) => (current.includes(code) ? current.filter((item) => item !== code) : [...current, code]));
  };

  const amountValue = Number(baseNightlyAmount);
  const amountValid = Number.isFinite(amountValue) && amountValue >= 1 && amountValue <= 999999999;
  const namesValid = Boolean(nameEn.trim() && nameFr.trim() && nameEn.length <= 200 && nameFr.length <= 200);
  const capacityValid = Number(maxAdults) >= 1 && Number(maxChildren) >= 0 && Number(bedrooms) >= 0 && Number(beds) >= 0 && Number(bathrooms) >= 0;
  const canSubmit = Boolean(hostId && categoryCode && namesValid && amountValid && capacityValid && !busy);

  const submit = async () => {
    if (!hostId || !canSubmit) {
      setFormError(translate('invalidForm'));
      return;
    }
    setBusy(true);
    setFormError(null);
    const input: TemplateInput = {
      hostId,
      categoryCode,
      inventoryModel,
      nameEn: nameEn.trim(),
      nameFr: nameFr.trim(),
      descriptionEn,
      descriptionFr,
      maxAdults: whole(maxAdults) || 1,
      maxChildren: whole(maxChildren),
      bedroomCount: whole(bedrooms),
      bedCount: whole(beds),
      bathroomCount: whole(bathrooms),
      accessibility,
      baseNightlyAmount,
      status,
      ...(target ? { expectedVersion: target.version } : {}),
    };
    const result: RequestResult<Template> = target
      ? await updateTemplate(client, target.id, input)
      : await createTemplate(client, input);
    setBusy(false);
    if (result.error || !result.data) {
      setFormError(message(result.error));
      return;
    }
    setOpenState(false);
    onSaved();
  };

  return {
    open,
    setOpen,
    target,
    openCreate,
    openEdit,
    nameEn,
    setNameEn,
    nameFr,
    setNameFr,
    descriptionEn,
    setDescriptionEn,
    descriptionFr,
    setDescriptionFr,
    categoryCode,
    setCategoryCode,
    categoryOptions: categories.map((item) => ({ value: item.code, label: (french ? item.nameFr : item.nameEn) || item.nameEn })),
    inventoryModel,
    setInventoryModel,
    modelOptions: [
      { value: 'WHOLE_UNIT', label: translate('inventoryModels.WHOLE_UNIT') },
      { value: 'POOLED_ROOM', label: translate('inventoryModels.POOLED_ROOM') },
    ],
    maxAdults,
    setMaxAdults,
    maxChildren,
    setMaxChildren,
    bedrooms,
    setBedrooms,
    beds,
    setBeds,
    bathrooms,
    setBathrooms,
    baseNightlyAmount,
    setBaseNightlyAmount,
    status,
    setStatus,
    statusOptions: [
      { value: 'ACTIVE', label: translate('statusActive') },
      { value: 'ARCHIVED', label: translate('statusArchived') },
    ],
    accessibilityOptions: TEMPLATE_ACCESSIBILITY.map((code) => ({
      code,
      label: translate(`accessibilityOptions.${code}`),
      checked: accessibility.includes(code),
      toggle: () => toggleAccessibility(code),
    })),
    busy,
    formError,
    canSubmit,
    submit,
    labels: {
      createTitle: translate('createTitle'),
      editTitle: translate('editTitle'),
      formHint: translate('formHint'),
      nameEn: translate('nameEn'),
      nameFr: translate('nameFr'),
      descriptionEn: translate('descriptionEn'),
      descriptionFr: translate('descriptionFr'),
      category: translate('category'),
      inventoryModel: translate('inventoryModel'),
      maxAdults: translate('maxAdults'),
      maxChildren: translate('maxChildren'),
      bedrooms: translate('bedrooms'),
      beds: translate('beds'),
      bathrooms: translate('bathrooms'),
      baseAmount: translate('baseAmount'),
      amountHint: translate('amountHint'),
      status: translate('status'),
      accessibility: translate('accessibility'),
      save: translate('save'),
      cancel: translate('cancel'),
    },
  };
};

export type TemplateEditorView = ReturnType<typeof useTemplateEditor>;
