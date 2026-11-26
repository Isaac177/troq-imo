import { NavLink } from 'react-router-dom';
import type { ModuleDefinition, SidebarSection } from '@/types/modules/module';
import { cn } from '@mungini/ui-web';

type SidebarNavProps = {
  modules: ModuleDefinition[];
  label: (key: string) => string;
  onNavigate: () => void;
};

const sections = (modules: ModuleDefinition[]): SidebarSection[] =>
  modules.reduce<SidebarSection[]>((result, module) => (result.includes(module.section) ? result : [...result, module.section]), []);

export const SidebarNav = ({ modules, label, onNavigate }: SidebarNavProps) => (
  <nav className="flex flex-col gap-5 px-3 py-4">
    {sections(modules).map((section) => (
      <div key={section} className="flex flex-col gap-1">
        <p className="px-2 text-micro text-muted-foreground">{label(section)}</p>
        {modules
          .filter((module) => module.section === section)
          .map((module) => {
            const Icon = module.icon;
            return (
              <NavLink
                key={module.id}
                to={module.route}
                end={module.route === '/'}
                onClick={onNavigate}
                className={({ isActive }) =>
                  cn(
                    'relative flex h-9 items-center gap-2 rounded-lg px-2 text-body transition-colors duration-200 ease-in-out',
                    isActive ? 'bg-accent font-semibold text-accent-foreground' : 'text-muted-foreground hover:bg-muted hover:text-foreground',
                  )
                }
              >
                {({ isActive }) => (
                  <>
                    {isActive ? <span className="absolute left-0 h-5 w-0.5 rounded-full bg-primary" aria-hidden="true" /> : null}
                    <Icon className="size-4 shrink-0" aria-hidden="true" />
                    <span className="truncate">{label(module.labelKey)}</span>
                  </>
                )}
              </NavLink>
            );
          })}
      </div>
    ))}
  </nav>
);
