'use client';

import Link from 'next/link';
import { Globe } from 'lucide-react';
import { Button, ThemeToggle } from '@mungini/ui-web';
import { NotificationsLink } from '@/components/notifications/NotificationsLink';
import { useSiteHeader } from '@/hooks/layout/useSiteHeader';

export default function SiteHeader() {
  const header = useSiteHeader();

  return (
    <header className="sticky top-0 z-20 border-b border-border bg-background/95 backdrop-blur">
      <div className="mx-auto flex h-[52px] w-full max-w-6xl items-center gap-2 px-3 md:px-6">
        <Link href="/" className="truncate text-title font-semibold">
          {header.labels.appName}
        </Link>
        <div className="flex-1" />
        <Button
          variant="ghost"
          size="icon"
          icon={<Globe />}
          aria-label={header.labels.language}
          title={header.labels.language}
          onClick={() => header.changeLanguage(header.language === 'fr' ? 'en' : 'fr')}
        />
        <ThemeToggle lightLabel={header.labels.light} darkLabel={header.labels.dark} />
        {header.status === 'authenticated' ? (
          <>
            <NotificationsLink />
            <Link href="/account" className="hidden text-body text-muted-foreground hover:text-foreground sm:block">
              {header.name || header.labels.account}
            </Link>
            <Button variant="ghost" size="sm" onClick={header.logout}>
              {header.labels.signOut}
            </Button>
          </>
        ) : (
          <>
            <Link href="/login" className="text-body text-muted-foreground hover:text-foreground">
              {header.labels.signIn}
            </Link>
            <Link href="/register" className="rounded-lg bg-primary px-3 py-2 text-body font-medium text-primary-foreground transition-colors duration-200 hover:bg-primary-hover">
              {header.labels.register}
            </Link>
          </>
        )}
      </div>
    </header>
  );
}
