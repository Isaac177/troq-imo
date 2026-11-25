'use client';

import { useHomeView } from '@/hooks/home/useHomeView';

export default function HomeView() {
  const { appName } = useHomeView();

  return (
    <main>
      <h1>{appName}</h1>
    </main>
  );
}
