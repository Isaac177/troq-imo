import type { StatusTone } from '@mungini/theme/tones';

export type SelectOption = {
  value: string;
  label: string;
  tone?: StatusTone;
  disabled?: boolean;
};
