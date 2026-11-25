export const textTones = {
  identity: 'text-tone-identity',
  place: 'text-tone-place',
  count: 'text-tone-count',
  people: 'text-tone-people',
  attention: 'text-tone-attention',
  problem: 'text-tone-problem',
} as const;

export type TextTone = keyof typeof textTones;

export const statusDotTones = {
  muted: 'bg-muted-foreground/30',
  neutral: 'bg-muted-foreground/50',
  primary: 'bg-primary',
  info: 'bg-info',
  success: 'bg-success',
  warning: 'bg-warning',
  danger: 'bg-destructive',
} as const;

export type StatusTone = keyof typeof statusDotTones;

export const statusPillTones = {
  muted: 'bg-muted',
  neutral: 'bg-muted',
  primary: 'bg-primary/10',
  info: 'bg-info/10',
  success: 'bg-success/10',
  warning: 'bg-warning/15',
  danger: 'bg-destructive/10',
} as const satisfies Record<StatusTone, string>;
