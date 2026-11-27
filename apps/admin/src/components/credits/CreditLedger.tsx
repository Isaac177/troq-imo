import { EmptyState, PagedTable } from '@mungini/ui-web';
import { useCreditLedger } from '@/hooks/credits/useCreditLedger';

type Props = { hostId: string | null; reloadToken: number; timezone: string };

export default function CreditLedger({ hostId, reloadToken, timezone }: Props) {
  const view = useCreditLedger(hostId, reloadToken, timezone);
  return (
    <div className="border-t border-border pt-3">
      <p className="text-label text-muted-foreground">{view.labels.title}</p>
      <PagedTable
        rows={view.rows}
        columns={view.columns}
        getRowId={(row) => row.id}
        page={view.page}
        pageSize={view.pageSize}
        total={view.total}
        onPageChange={view.setPage}
        ariaLabel={view.labels.title}
        loading={view.loading}
        error={view.error ?? undefined}
        emptyState={<EmptyState title={view.labels.empty} />}
      />
    </div>
  );
}
