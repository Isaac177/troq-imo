import { Link } from 'react-router-dom';
import { Percent, Wallet } from 'lucide-react';
import { Button, EmptyState, PagedTable, Section, Select, StatCard, StatusDot } from '@mungini/ui-web';
import { useHostCredits } from '@/hooks/credits/useHostCredits';

export default function CreditsView() {
  const view = useHostCredits();

  if (!view.hasHost) {
    return (
      <Section title={view.labels.title} description={view.labels.subtitle}>
        <EmptyState title={view.labels.noHost} action={<Button asChild variant="primary" size="sm"><Link to="/onboarding">{view.labels.noHostAction}</Link></Button>} />
      </Section>
    );
  }

  return (
    <div className="flex flex-col gap-4 md:gap-6">
      <Section
        title={view.labels.title}
        description={view.labels.subtitle}
        actions={
          view.hosts.length > 1 ? (
            <div className="w-full sm:w-[180px]">
              <Select options={view.hosts.map((host) => ({ value: host.hostId, label: host.displayName }))} value={view.hostId} onValueChange={view.setHostId} ariaLabel={view.labels.title} />
            </div>
          ) : null
        }
      >
        {view.error ? <p role="alert" className="text-label text-destructive">{view.error}</p> : null}
        {view.loading && !view.summary ? <p className="text-body text-muted-foreground">{view.labels.loading}</p> : null}
        {view.summary ? (
          <div className="flex flex-col gap-4">
            <div className="grid grid-cols-2 gap-3">
              <StatCard label={view.labels.balance} value={view.balance} icon={<Wallet />} hint={view.labels.lowThreshold + ' ' + view.threshold} />
              <StatCard label={view.labels.commissionRate} value={view.rate} icon={<Percent />} />
            </div>
            <div className="flex flex-wrap items-center gap-2 text-body">
              <span className="text-label text-muted-foreground">{view.labels.status}</span>
              <StatusDot tone={view.statusTone} label={view.status} />
            </div>
          </div>
        ) : null}
      </Section>

      <Section title={view.labels.ledger}>
        <PagedTable
          rows={view.rows}
          columns={view.columns}
          getRowId={(row) => row.id}
          page={view.page}
          pageSize={view.pageSize}
          total={view.total}
          onPageChange={view.setPage}
          ariaLabel={view.labels.ledger}
          loading={view.loading}
          emptyState={<EmptyState title={view.labels.empty} />}
        />
      </Section>
    </div>
  );
}
