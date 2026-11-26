import { Button, DateRangePicker, Input, Section, Select } from '@mungini/ui-web';
import { useCalendarView } from '@/hooks/calendar/useCalendarView';

export default function CalendarView() {
  const view = useCalendarView();
  return (
    <Section title={view.labels.title} description={view.labels.subtitle}>
      <div className="flex flex-col gap-3 sm:flex-row sm:items-end">
        <div className="flex min-w-0 flex-1 flex-col gap-1"><label className="text-label text-muted-foreground" htmlFor="calendar-search">{view.labels.search}</label><Input id="calendar-search" value={view.search} onChange={event => view.setSearch(event.target.value)} /></div>
        <div className="flex min-w-0 flex-1 flex-col gap-1"><span className="text-label text-muted-foreground">{view.labels.property}</span><Select options={view.options} value={view.propertyId} onValueChange={view.selectProperty} placeholder={view.labels.chooseProperty} ariaLabel={view.labels.property} /></div>
        <div className="flex min-w-0 flex-1 flex-col gap-1"><span className="text-label text-muted-foreground">{view.labels.range}</span><DateRangePicker value={view.range} onChange={view.setRange} ariaLabel={view.labels.range} /></div>
      </div>
      <div className="mt-3 flex items-center gap-2 text-label text-muted-foreground">
        <Button variant="ghost" size="sm" disabled={view.propertyPage <= 1} onClick={() => view.setPropertyPage(view.propertyPage - 1)}>{view.labels.previous}</Button><span>{view.labels.page} {view.propertyPage} / {view.propertyPages}</span><Button variant="ghost" size="sm" disabled={view.propertyPage >= view.propertyPages} onClick={() => view.setPropertyPage(view.propertyPage + 1)}>{view.labels.next}</Button>
      </div>
      {!view.validRange ? <p className="mt-3 text-label text-destructive">{view.labels.invalidRange}</p> : null}
      {view.error ? <p className="mt-3 text-label text-destructive">{view.labels.error}</p> : null}
      {view.loading ? <p className="mt-4 text-body text-muted-foreground">{view.labels.loading}</p> : null}
      {!view.loading && view.calendar ? (
        <div className="mt-5 space-y-6">
          <div className="space-y-4"><h2 className="text-title font-semibold">{view.labels.units}</h2>
            {view.calendar.units.length === 0 ? <p className="text-body text-muted-foreground">{view.labels.empty}</p> : null}
            {view.calendar.units.map(unit => <div key={unit.unitTypeId} className="space-y-2"><h3 className="text-body font-semibold">{view.french ? unit.nameFr || unit.nameEn : unit.nameEn || unit.nameFr}</h3><div className="grid gap-2 sm:grid-cols-2 lg:grid-cols-3">{unit.days.map(day => <div key={day.date} className="flex items-center justify-between gap-2 rounded-md bg-muted/40 px-3 py-2 text-label"><span className="font-medium tabular-nums">{view.day(day.date)}</span><span className="text-muted-foreground">{view.labels.allocated}: {day.allocated} / {day.capacity}</span><span className="tabular-nums">{day.price ? view.money(day.price) : '—'}</span></div>)}</div></div>)}
            <div className="flex items-center gap-2 text-label text-muted-foreground"><Button variant="ghost" size="sm" disabled={view.page <= 1} onClick={() => view.setPage(view.page - 1)}>{view.labels.previous}</Button><span>{view.labels.page} {view.page} / {view.unitPages}</span><Button variant="ghost" size="sm" disabled={view.page >= view.unitPages} onClick={() => view.setPage(view.page + 1)}>{view.labels.next}</Button></div>
          </div>
          <div className="border-t border-border pt-4"><h2 className="text-title font-semibold">{view.labels.reservations}</h2>{view.calendar.reservations.length === 0 ? <p className="mt-2 text-body text-muted-foreground">{view.labels.noReservations}</p> : <ul className="mt-2 divide-y divide-border">{view.calendar.reservations.map(item => <li key={item.id} className="flex flex-wrap justify-between gap-2 py-2 text-body"><span>{item.guestName || item.reference}</span><span className="tabular-nums">{view.day(item.checkIn)} – {view.day(item.checkOut)}</span><span className="tabular-nums">{view.money(item.total)}</span></li>)}</ul>}</div>
          <div className="border-t border-border pt-4"><h2 className="text-title font-semibold">{view.labels.blocks}</h2>{view.calendar.blocks.length === 0 ? <p className="mt-2 text-body text-muted-foreground">{view.labels.noBlocks}</p> : <ul className="mt-2 divide-y divide-border">{view.calendar.blocks.map(item => <li key={item.id} className="flex flex-wrap justify-between gap-2 py-2 text-body"><span>{item.reason}</span><span className="tabular-nums">{view.day(item.startDate)} – {view.day(item.endDate)}</span></li>)}</ul>}</div>
          <div className="border-t border-border pt-4"><h2 className="text-title font-semibold">{view.labels.restrictions}</h2>{view.calendar.restrictions.length === 0 ? <p className="mt-2 text-body text-muted-foreground">{view.labels.noRestrictions}</p> : <ul className="mt-2 divide-y divide-border">{view.calendar.restrictions.map(item => <li key={item.id} className="flex flex-wrap justify-between gap-2 py-2 text-body"><span>{view.restrictionName(item.kind)} {item.value ?? ''}</span><span className="tabular-nums">{item.startDate ? view.day(item.startDate) : '—'} – {item.endDate ? view.day(item.endDate) : '—'}</span></li>)}</ul>}</div>
        </div>
      ) : null}
    </Section>
  );
}
