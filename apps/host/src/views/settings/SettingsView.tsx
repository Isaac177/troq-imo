import { Button, Dialog, Input, Section, Select, Skeleton, Switch } from '@mungini/ui-web';
import { useHostSettings } from '@/hooks/settings/useHostSettings';

export default function SettingsView() {
  const view = useHostSettings();

  return (
    <div className="flex flex-col gap-4 md:gap-6">
      <Section
        title={view.labels.account}
        description={view.labels.subtitle}
        actions={<Button variant="outline" size="sm" onClick={view.openAccount}>{view.labels.editAccount}</Button>}
      >
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

      <Dialog
        open={view.editOpen}
        onOpenChange={view.setEditOpen}
        title={view.labels.editAccount}
        footer={
          <div className="flex justify-end gap-2">
            <Button variant="outline" disabled={view.accountBusy} onClick={() => view.setEditOpen(false)}>{view.labels.close}</Button>
            <Button variant="primary" loading={view.accountBusy} onClick={() => void view.saveAccount()}>{view.labels.save}</Button>
          </div>
        }
      >
        <div className="flex flex-col gap-3">
          {view.accountError ? <p role="alert" className="text-label text-destructive">{view.accountError}</p> : null}
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.firstName}
            <Input value={view.firstName} onChange={(event) => view.setFirstName(event.target.value)} aria-label={view.labels.firstName} />
          </label>
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.lastName}
            <Input value={view.lastName} onChange={(event) => view.setLastName(event.target.value)} aria-label={view.labels.lastName} />
          </label>
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.preferredLanguage}
            <Select options={view.languageOptions} value={view.preferredLanguage} onValueChange={view.setPreferredLanguage} ariaLabel={view.labels.preferredLanguage} />
          </label>
        </div>
      </Dialog>
    </div>
  );
}
