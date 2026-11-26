import { Section, Skeleton, Switch } from '@mungini/ui-web';
import { useHostSettings } from '@/hooks/settings/useHostSettings';

export default function SettingsView() {
  const view = useHostSettings();

  return (
    <div className="flex flex-col gap-4 md:gap-6">
      <Section title={view.labels.account} description={view.labels.subtitle}>
        <dl className="divide-y divide-border text-body">
          <div className="flex flex-wrap items-center justify-between gap-2 py-2">
            <dt className="text-muted-foreground">{view.labels.name}</dt>
            <dd className="font-medium">{view.account.name}</dd>
          </div>
          <div className="flex flex-wrap items-center justify-between gap-2 py-2">
            <dt className="text-muted-foreground">{view.labels.email}</dt>
            <dd className="break-all">{view.account.email}</dd>
          </div>
          <div className="flex flex-wrap items-center justify-between gap-2 py-2">
            <dt className="text-muted-foreground">{view.labels.phone}</dt>
            <dd className="tabular-nums">{view.account.phone}</dd>
          </div>
          <div className="flex flex-wrap items-center justify-between gap-2 py-2">
            <dt className="text-muted-foreground">{view.labels.city}</dt>
            <dd>{view.account.city}</dd>
          </div>
        </dl>
      </Section>

      <Section title={view.labels.notifications} description={view.labels.notificationsHint}>
        {view.error ? <p role="alert" className="text-label text-destructive">{view.error}</p> : null}
        {view.notice ? <p role="status" className="text-label text-muted-foreground">{view.notice}</p> : null}
        {view.loading ? <div className="space-y-2"><Skeleton shape="line" /><Skeleton shape="line" /></div> : null}
        {!view.loading && view.preferences.length === 0 ? <p className="text-body text-muted-foreground">{view.labels.notificationsHint}</p> : null}
        <ul className="divide-y divide-border">
          {view.preferences.map((preference) => (
            <li key={preference.eventKey} className="flex flex-col gap-2 py-3 sm:flex-row sm:items-center sm:justify-between">
              <p className="text-body font-medium text-foreground">{view.eventLabel(preference.eventKey)}</p>
              <div className="flex flex-wrap gap-3">
                <Switch checked={preference.inApp} onCheckedChange={(checked) => void view.toggle(preference, 'inApp', checked)} label={view.labels.inApp} disabled={view.savingKey === `${preference.eventKey}.inApp`} />
                <Switch checked={preference.email} onCheckedChange={(checked) => void view.toggle(preference, 'email', checked)} label={view.labels.emailChannel} disabled={view.savingKey === `${preference.eventKey}.email`} />
                <Switch checked={preference.push} onCheckedChange={(checked) => void view.toggle(preference, 'push', checked)} label={view.labels.push} disabled={view.savingKey === `${preference.eventKey}.push`} />
              </div>
            </li>
          ))}
        </ul>
      </Section>
    </div>
  );
}
