import { ShellHeader, ThemeToggle } from '@mungini/ui-web';
import { useHomeView } from '@/hooks/home/useHomeView';

export default function HomeView() {
  const { appName, lightLabel, darkLabel } = useHomeView();

  return (
    <main className="mungini-shell">
      <ShellHeader title={appName} actions={<ThemeToggle lightLabel={lightLabel} darkLabel={darkLabel} />} />
    </main>
  );
}
