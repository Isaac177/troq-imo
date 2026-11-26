import type { Metadata } from 'next';
import type { ReactNode } from 'react';
import AppProviders from '@/components/app/AppProviders';
import SiteHeader from '@/components/layout/SiteHeader';
import { i18n } from '@/i18n/instance';
import './globals.css';

export const metadata: Metadata = {
  title: i18n.t('appName'),
};

type RootLayoutProps = {
  children: ReactNode;
};

export default function RootLayout({ children }: RootLayoutProps) {
  return (
    <html lang={i18n.language}>
      <body className="min-h-dvh bg-background text-foreground">
        <AppProviders>
          <SiteHeader />
          {children}
        </AppProviders>
      </body>
    </html>
  );
}
