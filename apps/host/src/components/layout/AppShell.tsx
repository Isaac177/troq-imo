import { Outlet } from 'react-router-dom';
import { LogOut, Menu, X } from 'lucide-react';
import { Button, Select, ThemeToggle, cn } from '@mungini/ui-web';
import { SidebarNav } from './SidebarNav';
import { useAppShell } from '@/hooks/layout/useAppShell';

export default function AppShell() {
  const shell = useAppShell();

  return (
    <div className="min-h-dvh bg-background">
      <aside
        className={cn(
          'fixed inset-y-0 left-0 z-30 w-[240px] border-r border-border bg-card transition-transform duration-200 ease-in-out lg:translate-x-0',
          shell.menuOpen ? 'translate-x-0' : '-translate-x-full',
        )}
      >
        <div className="flex h-[52px] items-center justify-between border-b border-border px-4">
          <span className="text-title font-semibold">{shell.common('appName')}</span>
          <Button variant="ghost" size="icon" onClick={shell.closeMenu} aria-label={shell.common('closeMenu')} title={shell.common('closeMenu')} className="lg:hidden">
            <X aria-hidden="true" />
          </Button>
        </div>
        <div className="max-h-[calc(100dvh-52px)] overflow-y-auto">
          <SidebarNav modules={shell.modules} label={shell.host} onNavigate={shell.closeMenu} />
        </div>
      </aside>

      {shell.menuOpen ? <button type="button" className="fixed inset-0 z-20 bg-scrim/40 transition-opacity duration-200 ease-in-out lg:hidden" aria-label={shell.common('closeMenu')} onClick={shell.closeMenu} /> : null}

      <div className="lg:pl-[240px]">
        <header className="sticky top-0 z-10 flex h-[52px] items-center gap-2 border-b border-border bg-background/95 px-3 backdrop-blur md:px-5">
          <Button variant="ghost" size="icon" onClick={shell.toggleMenu} aria-label={shell.common('openMenu')} title={shell.common('openMenu')} className="lg:hidden">
            <Menu aria-hidden="true" />
          </Button>
          <div className="min-w-0 flex-1" />
          <div className="w-[130px] md:w-[160px]">
            <Select options={shell.cities} value={shell.city} onValueChange={shell.setCity} ariaLabel={shell.common('city')} placeholder={shell.common('city')} />
          </div>
          <div className="hidden w-[120px] sm:block">
            <Select
              options={[
                { value: 'en', label: shell.common('english') },
                { value: 'fr', label: shell.common('french') },
              ]}
              value={shell.language}
              onValueChange={shell.changeLanguage}
              ariaLabel={shell.common('language')}
            />
          </div>
          <ThemeToggle lightLabel={shell.common('switchToLight')} darkLabel={shell.common('switchToDark')} />
          <div className="hidden min-w-0 items-center gap-1 text-label text-muted-foreground md:flex">
            <span className="truncate">{shell.name}</span>
          </div>
          <Button variant="ghost" size="icon" onClick={shell.logout} aria-label={shell.common('signedInAs')} title={shell.common('signedInAs')}>
            <LogOut aria-hidden="true" />
          </Button>
        </header>
        <main className="px-3 py-4 md:px-8 md:py-6">
          <Outlet />
        </main>
      </div>
    </div>
  );
}
