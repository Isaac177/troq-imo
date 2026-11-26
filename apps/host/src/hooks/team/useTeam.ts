import { useEffect, useMemo, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { hostApi, type InvitationView, type MemberView, type PermissionView, type PresetView } from '@/services/host/host.api';

const PAGE_SIZE = 20;

export const useTeam = () => {
  const { t } = useTranslation('host');
  const translate = t as unknown as (key: string) => string;
  const { session, client } = useSession();
  const api = useMemo(() => hostApi(client), [client]);
  const hosts = session?.hosts ?? [];
  const [hostId, setHostId] = useState(hosts[0]?.hostId ?? '');
  const [members, setMembers] = useState<MemberView[]>([]);
  const [memberTotal, setMemberTotal] = useState(0);
  const [memberPage, setMemberPage] = useState(1);
  const [invitations, setInvitations] = useState<InvitationView[]>([]);
  const [invitationTotal, setInvitationTotal] = useState(0);
  const [invitationPage, setInvitationPage] = useState(1);
  const [permissions, setPermissions] = useState<PermissionView[]>([]);
  const [presets, setPresets] = useState<PresetView[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [notice, setNotice] = useState<string | null>(null);
  const [editing, setEditing] = useState<MemberView | null>(null);
  const [draft, setDraft] = useState<string[]>([]);
  const [inviteOpen, setInviteOpen] = useState(false);
  const [inviteError, setInviteError] = useState<string | null>(null);
  const [email, setEmail] = useState('');
  const [preset, setPreset] = useState('');

  useEffect(() => {
    if (!hostId && hosts[0]) setHostId(hosts[0].hostId);
  }, [hosts, hostId]);

  useEffect(() => {
    void api.catalog().then((result) => {
      if (result.error) return;
      setPermissions(result.data?.permissions.filter((item) => item.delegable) ?? []);
      setPresets(result.data?.presets.filter((item) => item.audience === 'HOST') ?? []);
    });
  }, [api]);

  useEffect(() => {
    if (!hostId) return;
    let active = true;
    setLoading(true);
    setError(null);
    void Promise.all([api.members(hostId, memberPage, PAGE_SIZE), api.invitations(hostId, invitationPage, PAGE_SIZE)]).then(([memberResult, invitationResult]) => {
      if (!active) return;
      setLoading(false);
      if (memberResult.error || invitationResult.error) {
        setError(t('error'));
        return;
      }
      setMembers(memberResult.data?.items ?? []);
      setMemberTotal(memberResult.data?.total ?? 0);
      setInvitations(invitationResult.data?.items ?? []);
      setInvitationTotal(invitationResult.data?.total ?? 0);
    });
    return () => {
      active = false;
    };
  }, [api, hostId, memberPage, invitationPage, t]);

  const openEditor = (member: MemberView) => {
    setEditing(member);
    setDraft(member.permissions);
  };

  const toggle = (permission: string, checked: boolean) => {
    setDraft((current) => (checked ? [...current, permission] : current.filter((value) => value !== permission)));
  };

  const savePermissions = async () => {
    if (!editing) return;
    const result = await api.setPermissions(hostId, editing.userId, draft);
    if (result.error) {
      setError(t('error'));
      return;
    }
    setMembers((current) => current.map((member) => (member.userId === editing.userId ? { ...member, permissions: draft } : member)));
    setEditing(null);
    setNotice(t('saved'));
  };

  const removeMember = async (member: MemberView) => {
    const result = await api.removeMember(hostId, member.userId);
    if (result.error) {
      setError(t('error'));
      return;
    }
    setMembers((current) => current.filter((item) => item.userId !== member.userId));
    setMemberTotal((current) => Math.max(0, current - 1));
    setNotice(t('removed'));
  };

  const sendInvite = async () => {
    const selected = presets.find((item) => item.code === preset);
    if (!selected) return;
    setInviteError(null);
    const result = await api.invite(hostId, email.trim(), selected.permissions);
    if (result.error) {
      setInviteError(result.error === 'INVITEE_CITY_MISMATCH' ? t('inviteCityMismatch') : t('error'));
      return;
    }
    setInviteOpen(false);
    setEmail('');
    setPreset('');
    setNotice(t('movedToInvitations'));
    const refreshed = await api.invitations(hostId, invitationPage, PAGE_SIZE);
    setInvitations(refreshed.data?.items ?? []);
    setInvitationTotal(refreshed.data?.total ?? 0);
  };

  const revoke = async (invitation: InvitationView) => {
    const result = await api.revokeInvitation(hostId, invitation.id);
    if (result.error) {
      setError(t('error'));
      return;
    }
    setInvitations((current) => current.filter((item) => item.id !== invitation.id));
    setInvitationTotal((current) => Math.max(0, current - 1));
  };

  return {
    hosts,
    hostId,
    setHostId,
    members,
    memberTotal,
    memberPage,
    setMemberPage,
    invitations,
    invitationTotal,
    invitationPage,
    setInvitationPage,
    permissions,
    presets,
    loading,
    error,
    notice,
    editing,
    draft,
    toggle,
    openEditor,
    closeEditor: () => setEditing(null),
    savePermissions,
    removeMember,
    inviteOpen,
    inviteError,
    setInviteOpen,
    email,
    setEmail,
    preset,
    setPreset,
    sendInvite,
    revoke,
    pageSize: PAGE_SIZE,
    labels: {
      title: t('teamTitle'),
      subtitle: t('teamSubtitle'),
      members: t('members'),
      invitations: t('invitations'),
      member: t('member'),
      owner: t('owner'),
      permissions: t('permissions'),
      save: t('savePermissions'),
      remove: t('removeMember'),
      invite: t('invite'),
      email: t('inviteEmail'),
      inviteSend: t('inviteSend'),
      inviteCityHint: t('inviteCityHint'),
      inviteEmpty: t('inviteEmpty'),
      membersEmpty: t('membersEmpty'),
      cancel: t('cancel'),
      revoke: t('revoke'),
      error: t('error'),
      permissionLabels: (permission: string) => translate(`permissionLabels.${permission}`),
      presetLabels: (code: string) => translate(`presetLabels.${code}`),
    },
  };
};
