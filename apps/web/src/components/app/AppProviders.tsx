'use client';

import type { ReactNode } from 'react';
import { I18nextProvider } from '@mungini/i18n';
import { SessionProvider } from '@mungini/session';
import { i18n } from '@/i18n/instance';
import { sessionClient } from '@/services/session/sessionClient';
import UiCopyBridge from '@/components/app/UiCopyBridge';

type AppProvidersProps = {
  children: ReactNode;
};

export default function AppProviders({ children }: AppProvidersProps) {
  return (
    <I18nextProvider i18n={i18n}>
      <UiCopyBridge>
        <SessionProvider client={sessionClient}>{children}</SessionProvider>
      </UiCopyBridge>
    </I18nextProvider>
  );
}
