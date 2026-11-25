'use client';

import type { ReactNode } from 'react';
import { I18nextProvider } from '@mungini/i18n';
import { i18n } from '@/i18n/instance';

type AppProvidersProps = {
  children: ReactNode;
};

export default function AppProviders({ children }: AppProvidersProps) {
  return <I18nextProvider i18n={i18n}>{children}</I18nextProvider>;
}
