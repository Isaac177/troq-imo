import { useEffect, useState, type FormEvent } from 'react';
import { useSearchParams } from 'react-router-dom';
import { ImageOff } from 'lucide-react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { StatusDot, type TableColumn } from '@mungini/ui-web';
import { formatCdf, formatDateTime } from '@mungini/utils';
import { listCategories, listCities } from '@/services/properties/properties.api';
import { listTemplates } from '@/services/templates/templates.api';
import type { Template, TemplateCategory } from '@/types/templates/templates';

const PAGE_SIZE = 10;
const STATUSES = ['ALL', 'ACTIVE', 'ARCHIVED'];
const KNOWN_ERRORS = [
  'FORBIDDEN',
  'HOST_NOT_MEMBER',
  'HOST_NOT_FOUND',
  'HOST_NOT_ACTIVE',
  'TEMPLATE_NOT_FOUND',
  'TEMPLATE_AMOUNT_INVALID',
  'TEMPLATE_STATUS_INVALID',
  'TEMPLATE_VERSION_CONFLICT',
];

const positiveInt = (value: string | null): number => {
  const parsed = Number(value ?? '1');
  return Number.isFinite(parsed) && parsed >= 1 ? Math.floor(parsed) : 1;
};

export const useHostTemplates = () => {
  const { t, i18n } = useTranslation('hostTemplates');
  const translate = t as unknown as (key: string, options?: Record<string, unknown>) => string;
  const { session, client, city, can } = useSession();
  const [params, setParams] = useSearchParams();
  const hosts = (session?.hosts ?? []).filter((host) => can('properties.read', host.hostId));
  const requestedHost = params.get('host') ?? '';
  const hostId = hosts.some((host) => host.hostId === requestedHost) ? requestedHost : hosts[0]?.hostId ?? '';
  const statusParam = params.get('status') ?? 'ALL';
  const status = STATUSES.includes(statusParam) ? statusParam : 'ALL';
  const category = params.get('category') ?? 'ALL';
  const page = positiveInt(params.get('page'));
  const search = params.get('q') ?? '';
  const french = i18n.language.startsWith('fr');
  const [searchInput, setSearchInput] = useState(search);
  const [categories, setCategories] = useState<TemplateCategory[]>([]);
  const [timezone, setTimezone] = useState('UTC');
  const [items, setItems] = useState<Template[]>([]);
  const [total, setTotal] = useState(0);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [forbidden, setForbidden] = useState(false);
  const [notice, setNotice] = useState<string | null>(null);
  const [version, setVersion] = useState(0);

  const message = (code: string | null) => (code && KNOWN_ERRORS.includes(code) ? translate(`errors.${code}`) : translate('errors.generic'));

  const update = (values: Record<string, string | null>) => {
    const next = new URLSearchParams(params);
    for (const [key, value] of Object.entries(values)) {
      if (value) next.set(key, value);
      else next.delete(key);
    }
    setParams(next, { replace: true });
  };

  useEffect(() => {
    setSearchInput(search);
  }, [search]);

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
    let active = true;
    void listCities(client).then(({ data }) => {
      const match = data?.data.find((item) => item.code === city);
      if (active) setTimezone(match?.timezone ?? 'UTC');
    }).catch(() => undefined);
    return () => {
      active = false;
    };
  }, [client, city]);

  useEffect(() => {
    setError(null);
    setForbidden(false);
    if (!hostId) {
      setLoading(false);
      setItems([]);
      setTotal(0);
      return;
    }
    let active = true;
    setLoading(true);
    void listTemplates(client, { hostId, search, status, categoryCode: category, page, pageSize: PAGE_SIZE }).then((result) => {
      if (!active) return;
      setLoading(false);
      if (result.error || !result.data) {
        if (result.error === 'FORBIDDEN' || result.error === 'HOST_NOT_MEMBER') setForbidden(true);
        else setError(message(result.error));
        setItems([]);
        setTotal(0);
        return;
      }
      setItems(result.data.items);
      setTotal(result.data.total);
    });
    return () => {
      active = false;
    };
  }, [client, hostId, search, status, category, page, version, translate]);

  const nameOf = (template: Template) => {
    const primary = french ? template.nameFr : template.nameEn;
    return primary || template.nameEn || template.nameFr;
  };
  const otherNameOf = (template: Template) => (french ? template.nameEn : template.nameFr);
  const categoryLabel = (code: string) => {
    const match = categories.find((item) => item.code === code);
    if (!match) return code;
    return (french ? match.nameFr : match.nameEn) || match.nameEn;
  };
  const modelLabel = (model: string) => {
    const key = `inventoryModels.${model}`;
    const label = translate(key);
    return label === key ? model : label;
  };
  const statusLabel = (value: string) => translate(value === 'ARCHIVED' ? 'statusArchived' : 'statusActive');
  const cover = (template: Template) => (
    template.coverUrl
      ? <img src={template.coverUrl} alt="" className="size-10 shrink-0 rounded-lg object-cover" />
      : <span className="flex size-10 shrink-0 items-center justify-center rounded-lg bg-muted text-muted-foreground"><ImageOff className="size-4" aria-hidden="true" /></span>
  );

  const columns: TableColumn<Template>[] = [
    {
      key: 'name',
      header: translate('name'),
      cell: (row) => (
        <div className="flex min-w-0 items-center gap-2">
          {cover(row)}
          <div className="min-w-0">
            <p className="truncate font-medium text-foreground">{nameOf(row)}</p>
            <p className="truncate text-label text-muted-foreground">{otherNameOf(row)}</p>
          </div>
        </div>
      ),
    },
    { key: 'model', header: translate('inventoryModel'), hideBelow: 'sm', cell: (row) => modelLabel(row.inventoryModel) },
    { key: 'category', header: translate('category'), hideBelow: 'sm', cell: (row) => categoryLabel(row.categoryCode) },
    { key: 'capacity', header: translate('capacity'), hideBelow: 'md', cell: (row) => translate('capacityValue', { adults: row.maxAdults, children: row.maxChildren }) },
    { key: 'photos', header: translate('photos'), hideBelow: 'sm', align: 'right', cell: (row) => <span className="tabular-nums">{row.imageCount}</span> },
    { key: 'amount', header: translate('baseAmount'), align: 'right', cell: (row) => <span className="tabular-nums">{formatCdf(row.baseNightlyAmount, i18n.language)}</span> },
    { key: 'updated', header: translate('updated'), hideBelow: 'lg', cell: (row) => <span className="whitespace-nowrap tabular-nums">{formatDateTime(row.updatedAt, timezone)}</span> },
    { key: 'status', header: translate('status'), cell: (row) => <StatusDot tone={row.status === 'ARCHIVED' ? 'muted' : 'success'} label={statusLabel(row.status)} /> },
  ];

  return {
    hosts,
    hasHost: hosts.length > 0,
    hostId,
    setHostId: (value: string) => {
      setNotice(null);
      update({ host: value, page: null });
    },
    categories,
    categoryOptions: [
      { value: 'ALL', label: translate('allCategories') },
      ...categories.map((item) => ({ value: item.code, label: (french ? item.nameFr : item.nameEn) || item.nameEn })),
    ],
    statusOptions: [
      { value: 'ALL', label: translate('allStatuses') },
      { value: 'ACTIVE', label: translate('statusActive') },
      { value: 'ARCHIVED', label: translate('statusArchived') },
    ],
    status,
    setStatus: (value: string) => update({ status: value === 'ALL' ? null : value, page: null }),
    category,
    setCategory: (value: string) => update({ category: value === 'ALL' ? null : value, page: null }),
    searchInput,
    setSearchInput,
    onSearchSubmit: (event: FormEvent<HTMLFormElement>) => {
      event.preventDefault();
      update({ q: searchInput.trim() || null, page: null });
    },
    clearFilters: () => {
      setSearchInput('');
      update({ q: null, status: null, category: null, page: null });
    },
    hasFilters: Boolean(search) || status !== 'ALL' || category !== 'ALL',
    items,
    columns,
    total,
    page,
    pageSize: PAGE_SIZE,
    setPage: (value: number) => update({ page: value > 1 ? String(value) : null }),
    loading,
    error,
    forbidden,
    notice,
    canWrite: can('properties.configure', hostId),
    reload: () => setVersion((current) => current + 1),
    afterSave: () => {
      setNotice(translate('saved'));
      setVersion((current) => current + 1);
    },
    timezone,
    labels: {
      title: translate('title'),
      subtitle: translate('subtitle'),
      newTemplate: translate('newTemplate'),
      searchLabel: translate('searchLabel'),
      searchPlaceholder: translate('searchPlaceholder'),
      searchAction: translate('searchAction'),
      clearFilters: translate('clearFilters'),
      status: translate('status'),
      category: translate('category'),
      baseAmount: translate('baseAmount'),
      photos: translate('photos'),
      inventoryModel: translate('inventoryModel'),
      capacity: translate('capacity'),
      updated: translate('updated'),
      name: translate('name'),
      empty: translate('empty'),
      emptyHint: translate('emptyHint'),
      error: translate('errors.generic'),
      forbidden: translate('forbidden'),
      loading: translate('loading'),
      noHost: translate('noHost'),
      noHostAction: translate('noHostAction'),
      saved: translate('saved'),
    },
  };
};

export type HostTemplatesView = ReturnType<typeof useHostTemplates>;
