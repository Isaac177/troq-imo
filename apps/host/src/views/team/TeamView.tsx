import { Plus } from 'lucide-react';
import { Button, Checkbox, Dialog, EmptyState, Input, PagedTable, Section, Select, type TableColumn } from '@mungini/ui-web';
import { useTeam } from '@/hooks/team/useTeam';
import type { InvitationView, MemberView } from '@/services/host/host.api';

export default function TeamView() {
  const view = useTeam();

  const memberColumns: TableColumn<MemberView>[] = [
    {
      key: 'member',
      header: view.labels.member,
      cell: (row) => (
        <div className="min-w-0">
          <p className="truncate font-medium text-foreground">{[row.firstName, row.lastName].filter(Boolean).join(' ') || row.email || row.userId}</p>
          {row.email ? <p className="truncate text-label text-muted-foreground">{row.email}</p> : null}
        </div>
      ),
    },
    {
      key: 'role',
      header: view.labels.owner,
      cell: (row) => (row.isOwner ? view.labels.owner : ''),
      hideBelow: 'sm',
    },
    {
      key: 'permissions',
      header: view.labels.permissions,
      cell: (row) => <span className="tabular-nums text-muted-foreground">{row.permissions.length}</span>,
      align: 'right',
      hideBelow: 'sm',
    },
    {
      key: 'actions',
      header: '',
      align: 'right',
      cell: (row) =>
        row.isOwner ? null : (
          <div className="flex justify-end gap-1">
            <Button variant="ghost" size="sm" onClick={() => view.openEditor(row)}>
              {view.labels.permissions}
            </Button>
            <Button variant="ghost" size="sm" onClick={() => void view.removeMember(row)}>
              {view.labels.remove}
            </Button>
          </div>
        ),
    },
  ];

  const invitationColumns: TableColumn<InvitationView>[] = [
    { key: 'email', header: view.labels.email, cell: (row) => row.email },
    { key: 'permissions', header: view.labels.permissions, cell: (row) => row.permissions.length, align: 'right', hideBelow: 'sm' },
    {
      key: 'actions',
      header: '',
      align: 'right',
      cell: (row) => (
        <Button variant="ghost" size="sm" onClick={() => void view.revoke(row)}>
          {view.labels.revoke}
        </Button>
      ),
    },
  ];

  return (
    <div className="flex flex-col gap-4 md:gap-6">
      <Section
        title={view.labels.title}
        description={view.labels.subtitle}
        actions={
          <div className="flex items-center gap-2">
            {view.hosts.length > 1 ? (
              <div className="w-[160px]">
                <Select
                  options={view.hosts.map((host) => ({ value: host.hostId, label: host.displayName }))}
                  value={view.hostId}
                  onValueChange={view.setHostId}
                  ariaLabel={view.labels.title}
                />
              </div>
            ) : null}
            <Button variant="primary" size="sm" icon={<Plus />} collapseLabel onClick={() => view.setInviteOpen(true)} aria-label={view.labels.invite} title={view.labels.invite}>
              {view.labels.invite}
            </Button>
          </div>
        }
      >
        {view.error ? <p className="text-label text-destructive">{view.error}</p> : null}
        {view.notice ? <p className="text-label text-muted-foreground">{view.notice}</p> : null}
        <PagedTable
          rows={view.members}
          columns={memberColumns}
          getRowId={(row) => row.userId}
          page={view.memberPage}
          pageSize={view.pageSize}
          total={view.memberTotal}
          onPageChange={view.setMemberPage}
          ariaLabel={view.labels.members}
          loading={view.loading}
          emptyState={<EmptyState title={view.labels.membersEmpty} />}
        />
      </Section>

      <Section title={view.labels.invitations}>
        <PagedTable
          rows={view.invitations}
          columns={invitationColumns}
          getRowId={(row) => row.id}
          page={view.invitationPage}
          pageSize={view.pageSize}
          total={view.invitationTotal}
          onPageChange={view.setInvitationPage}
          ariaLabel={view.labels.invitations}
          loading={view.loading}
          emptyState={<EmptyState title={view.labels.inviteEmpty} />}
        />
      </Section>

      <Dialog
        open={view.editing !== null}
        onOpenChange={(open) => (open ? undefined : view.closeEditor())}
        title={view.labels.permissions}
        footer={
          <div className="flex justify-end gap-2">
            <Button variant="ghost" onClick={view.closeEditor}>
              {view.labels.cancel}
            </Button>
            <Button variant="primary" onClick={() => void view.savePermissions()}>
              {view.labels.save}
            </Button>
          </div>
        }
      >
        <div className="grid grid-cols-1 gap-1 sm:grid-cols-2">
          {view.permissions.map((permission) => (
            <Checkbox
              key={permission.name}
              checked={view.draft.includes(permission.name)}
              onCheckedChange={(checked) => view.toggle(permission.name, checked)}
              label={view.labels.permissionLabels(permission.name)}
            />
          ))}
        </div>
      </Dialog>

      <Dialog
        open={view.inviteOpen}
        onOpenChange={view.setInviteOpen}
        title={view.labels.invite}
        footer={
          <div className="flex justify-end gap-2">
            <Button variant="ghost" onClick={() => view.setInviteOpen(false)}>
              {view.labels.cancel}
            </Button>
            <Button variant="primary" onClick={() => void view.sendInvite()} disabled={!view.email.trim() || !view.preset}>
              {view.labels.inviteSend}
            </Button>
          </div>
        }
      >
        <div className="flex flex-col gap-3">
          <p className="text-label text-muted-foreground">{view.labels.inviteCityHint}</p>
          {view.inviteError ? <p role="alert" className="text-label text-destructive">{view.inviteError}</p> : null}
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.email}
            <Input type="email" value={view.email} onChange={(event) => view.setEmail(event.target.value)} />
          </label>
          <label className="flex flex-col gap-1 text-label text-muted-foreground">
            {view.labels.permissions}
            <Select
              options={view.presets.map((preset) => ({ value: preset.code, label: view.labels.presetLabels(preset.code) }))}
              value={view.preset}
              onValueChange={view.setPreset}
              placeholder={view.labels.permissions}
              ariaLabel={view.labels.permissions}
            />
          </label>
        </div>
      </Dialog>
    </div>
  );
}
