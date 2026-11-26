import { CalendarCheck, CircleDollarSign, Inbox, Wallet } from 'lucide-react';
import { DateRangePicker, PagedTable, Section, Skeleton, StatCard } from '@mungini/ui-web';
import { useAdminDashboard } from '@/hooks/dashboard/useAdminDashboard';

export default function DashboardView() {
  const view = useAdminDashboard();

  return (
    <div className="flex flex-col gap-4 md:gap-6">
      <div className="flex flex-col gap-3 sm:flex-row sm:items-end sm:justify-between">
        <div className="min-w-0">
          <h2 className="text-title font-semibold text-foreground">{view.labels.title}</h2>
          <p className="mt-0.5 text-body text-muted-foreground">{view.labels.subtitle}</p>
        </div>
        <div className="w-full sm:w-[220px]">
          <DateRangePicker value={view.range} onChange={view.setRange} />
        </div>
      </div>
      {view.error ? <p className="text-label text-destructive">{view.error}</p> : null}
      {view.loading && !view.totals ? (
        <div className="grid grid-cols-2 gap-2 md:grid-cols-4 md:gap-3">
          <Skeleton className="h-[70px]" />
          <Skeleton className="h-[70px]" />
          <Skeleton className="h-[70px]" />
          <Skeleton className="h-[70px]" />
        </div>
      ) : (
        <div className="grid grid-cols-2 gap-2 md:grid-cols-4 md:gap-3">
          <StatCard label={view.labels.revenue} value={view.values.revenue} icon={<CircleDollarSign />} />
          <StatCard label={view.labels.bookings} value={view.values.bookings} icon={<CalendarCheck />} />
          <StatCard label={view.labels.pending} value={view.values.pending} icon={<Inbox />} />
          <StatCard label={view.labels.credit} value={view.values.credit} icon={<Wallet />} />
        </div>
      )}
      <Section title={view.labels.hosts}>
        <PagedTable
          rows={view.hosts}
          columns={view.columns}
          getRowId={(row) => row.hostId}
          page={view.page}
          pageSize={view.pageSize}
          total={view.total}
          onPageChange={view.setPage}
          ariaLabel={view.labels.hosts}
          loading={view.loading}
        />
      </Section>
    </div>
  );
}
