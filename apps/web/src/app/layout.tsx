import type { Metadata } from 'next';
import type { ReactNode } from 'react';
import AppProviders from '@/components/app/AppProviders';
import SiteHeader from '@/components/layout/SiteHeader';
import { cookies } from 'next/headers';
import { createServerI18n } from '@mungini/i18n/server';
import './globals.css';

export const metadata: Metadata = {
  title: createServerI18n('en').t('appName'),
};

type RootLayoutProps = {
  children: ReactNode;
};

export default async function RootLayout({ children }: RootLayoutProps) {
  const language = (await cookies()).get('mungini-language')?.value === 'fr' ? 'fr' : 'en';
  return (
    <html lang={language}>
      <body className="min-h-dvh bg-background text-foreground">
        <AppProviders>
          <SiteHeader />
          {children}
        </AppProviders>
      </body>
    </html>
  );
}
