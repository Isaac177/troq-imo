import { useEffect, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { assignCase, listAssignees } from '@/services/cases/cases.api';
import type { AdminCase, CaseAssignee } from '@/types/cases/cases';

export const useCaseAssignment = (item: AdminCase, onChanged: () => void) => {
  const { t } = useTranslation('adminCases');
  const { client } = useSession();
  const [open, setOpen] = useState(false);
  const [search, setSearch] = useState('');
  const [page, setPage] = useState(1);
  const [total, setTotal] = useState(0);
  const [items, setItems] = useState<CaseAssignee[]>([]);
  const [selected, setSelected] = useState('');
  const [loading, setLoading] = useState(false);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState('');
  useEffect(() => {
    if (!open) return;
    let active = true;
    setLoading(true);
    setError('');
    void listAssignees(client, search, page).then((result) => {
      if (!active) return;
      setLoading(false);
      if (result.error || !result.data) { setError(t('assigneesError')); return; }
      setItems(result.data.items);
      setTotal(result.data.total);
    }).catch(() => { if (active) { setLoading(false); setError(t('assigneesError')); } });
    return () => { active = false; };
  }, [client, open, search, page, t]);
  const changeSearch = (value: string) => { setSearch(value); setPage(1); setSelected(''); };
  const changePage = (value: number) => { setPage(value); setSelected(''); };
  const submit = async () => {
    if (!selected) return;
    setBusy(true);
    setError('');
    const result = await assignCase(client, item.id, item.version, selected === 'unassigned' ? null : selected);
    setBusy(false);
    if (result.error || !result.data) { setError(t('assignmentError')); return; }
    setOpen(false);
    setSelected('');
    onChanged();
  };
  return { open, setOpen, search, changeSearch, page, changePage, total, selected, setSelected, loading, busy, error, submit,
    options: [{ value: 'unassigned', label: t('unassigned') }, ...items.map((person) => ({ value: person.userId, label: [person.name, person.email].filter(Boolean).join(' · ') || t('unnamedStaff') }))],
    labels: { assign: t('assign'), assignTitle: t('assignTitle'), search: t('searchStaff'), staff: t('staff'), cancel: t('dismiss'), save: t('saveAssignment'), previous: t('previous'), next: t('next') },
  };
};
