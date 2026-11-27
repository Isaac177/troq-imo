import { Button, Input, PhoneField, Section, Select, Skeleton } from '@mungini/ui-web';
import { useHostContact } from '@/hooks/settings/useHostContact';

export const HostContactSection = () => {
  const view = useHostContact();
  return view.available ? (
    <Section title={view.labels.title} description={view.labels.hint}>
      <div className="flex flex-col gap-3">
        <label className="flex flex-col gap-1 text-label text-muted-foreground">
          {view.labels.host}
          <Select options={view.hosts} value={view.hostId} onValueChange={view.setHostId} ariaLabel={view.labels.host} disabled={view.saving} />
        </label>
        {view.loading ? <Skeleton shape="line" /> : null}
        {view.error ? <p role="alert" className="text-label text-destructive">{view.error}</p> : null}
        {view.notice ? <p role="status" className="text-label text-muted-foreground">{view.notice}</p> : null}
        <label className="flex flex-col gap-1 text-label text-muted-foreground" htmlFor="host-contact-email">
          {view.labels.email}
          <Input id="host-contact-email" type="email" value={view.email} onChange={(event) => view.setEmail(event.target.value)} disabled={view.disabled} />
        </label>
        <label className="flex flex-col gap-1 text-label text-muted-foreground" htmlFor="host-contact-phone">
          {view.labels.phone}
          <PhoneField id="host-contact-phone" value={view.phone} onChange={view.setPhone} disabled={view.disabled} />
        </label>
        <div className="flex justify-end"><Button variant="primary" disabled={view.disabled} loading={view.saving} onClick={() => void view.save()}>{view.labels.save}</Button></div>
      </div>
    </Section>
  ) : null;
};
