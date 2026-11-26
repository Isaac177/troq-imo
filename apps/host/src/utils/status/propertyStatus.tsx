import { StatusDot } from '@mungini/ui-web';
import type { StatusTone } from '@mungini/theme/tones';

const tones: Record<string, StatusTone> = {
  DRAFT: 'neutral',
  APPROVED: 'info',
  PUBLISHED: 'success',
  PAUSED: 'warning',
  SUSPENDED: 'danger',
  ARCHIVED: 'muted',
};

export const propertyStatusView = (status: string, translate: (key: string) => string) => (
  <StatusDot tone={tones[status] ?? 'muted'} label={translate(`propertyStatuses.${status}`)} />
);
