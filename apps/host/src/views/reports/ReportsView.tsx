import { CalendarCheck, CircleDollarSign, Star, Users } from 'lucide-react';
import { DateRangePicker, EmptyState, Section, Select, Skeleton, StatCard } from '@mungini/ui-web';
import { useReports } from '@/hooks/reports/useReports';

export default function ReportsView() {
  const view = useReports();
  return (
    <div className="flex flex-col gap-4 md:gap-6">
      <div className="flex flex-col gap-3 sm:flex-row sm:items-end sm:justify-between">
        <div><h2 className="text-title font-semibold text-foreground">{view.labels.title}</h2><p className="text-body text-muted-foreground">{view.labels.subtitle}</p></div>
        <div className="flex w-full flex-col gap-2 sm:w-auto sm:flex-row">
          {view.hosts.length > 1 ? <div className="w-full sm:w-[180px]"><Select options={view.hosts} value={view.hostId} onValueChange={view.setHostId} ariaLabel={view.labels.host} /></div> : null}
          {view.propertyOptions.length > 1 ? <div className="w-full sm:w-[200px]"><Select options={view.propertyOptions} value={view.propertyId} onValueChange={view.setPropertyId} ariaLabel={view.labels.property} /></div> : null}
          <div className="w-full sm:w-[240px]"><DateRangePicker value={view.range} onChange={view.setRange} ariaLabel={view.labels.dates} clearable /></div>
        </div>
      </div>
      {!view.hostId ? <EmptyState title={view.labels.empty} /> : null}
      {view.error ? <p role="alert" className="text-label text-destructive">{view.labels.error}</p> : null}
      {view.loading && !view.summary ? <Skeleton className="h-[100px]" /> : view.summary ? (
        <>
          <div className="grid grid-cols-2 gap-2 md:grid-cols-4 md:gap-3">
            <StatCard label={view.labels.revenue} value={view.revenue} icon={<CircleDollarSign />} />
            <StatCard label={view.labels.reservations} value={String(view.summary.reservations)} icon={<CalendarCheck />} />
            <StatCard label={view.labels.reviews} value={String(view.summary.approvedReviews)} icon={<Users />} />
            <StatCard label={view.labels.rating} value={view.rating} icon={<Star />} />
          </div>
          <Section title={view.labels.reservations}>
            <dl className="divide-y divide-border text-body">
              <div className="flex justify-between gap-3 py-2"><dt>{view.labels.confirmed}</dt><dd className="tabular-nums">{view.summary.confirmed}</dd></div>
              <div className="flex justify-between gap-3 py-2"><dt>{view.labels.checkedIn}</dt><dd className="tabular-nums">{view.summary.checkedIn}</dd></div>
              <div className="flex justify-between gap-3 py-2"><dt>{view.labels.checkedOut}</dt><dd className="tabular-nums">{view.summary.checkedOut}</dd></div>
              <div className="flex justify-between gap-3 py-2"><dt>{view.labels.cancelled}</dt><dd className="tabular-nums">{view.summary.cancelled}</dd></div>
              <div className="flex justify-between gap-3 py-2"><dt>{view.labels.noShow}</dt><dd className="tabular-nums">{view.summary.noShow}</dd></div>
            </dl>
          </Section>
        </>
      ) : null}
    </div>
  );
}
