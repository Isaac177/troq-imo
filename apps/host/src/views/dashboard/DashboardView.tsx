import { Link } from 'react-router-dom';
import { CalendarCheck, CircleDollarSign, DoorOpen, Inbox } from 'lucide-react';
import { DateRangePicker, EmptyState, Select, Skeleton, StatCard } from '@mungini/ui-web';
import { useDashboard } from '@/hooks/dashboard/useDashboard';

export default function DashboardView() {
  const view = useDashboard();

  if (view.hosts.length === 0) {
    return (
      <EmptyState
        title={view.labels.noHost}
        action={
          <Link to="/onboarding" className="text-body font-medium text-primary underline-offset-4 hover:underline">
            {view.labels.noHostAction}
          </Link>
        }
      />
    );
  }

  return (
    <div className="flex flex-col gap-4 md:gap-6">
      <div className="flex flex-col gap-3 sm:flex-row sm:items-end sm:justify-between">
        <div className="min-w-0">
          <h2 className="text-title font-semibold text-foreground">{view.labels.title}</h2>
          <p className="mt-0.5 text-body text-muted-foreground">{view.labels.subtitle}</p>
        </div>
        <div className="flex flex-col gap-2 sm:flex-row">
          {view.hosts.length > 1 ? (
            <div className="w-full sm:w-[180px]">
              <Select
                options={view.hosts.map((host) => ({ value: host.hostId, label: host.displayName }))}
                value={view.hostId}
                onValueChange={view.setHostId}
                ariaLabel={view.labels.title}
              />
            </div>
          ) : null}
          <div className="w-full sm:w-[220px]">
            <DateRangePicker value={view.range} onChange={view.setRange} />
          </div>
        </div>
      </div>
      {view.error ? <p className="text-label text-destructive">{view.error}</p> : null}
      {view.loading && !view.figures ? (
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
          <StatCard label={view.labels.occupancy} value={view.values.occupancy} icon={<DoorOpen />} />
          <StatCard label={view.labels.pending} value={view.values.pending} icon={<Inbox />} />
        </div>
      )}
    </div>
  );
}
