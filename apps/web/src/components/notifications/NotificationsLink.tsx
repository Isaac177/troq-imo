'use client';

import Link from 'next/link';
import { Bell } from 'lucide-react';
import { Button } from '@mungini/ui-web';
import { useNotificationsLink } from '@/hooks/notifications/useNotificationsLink';

export const NotificationsLink = () => {
  const link = useNotificationsLink();
  return (
    <Button asChild variant="ghost" size="icon" aria-label={link.open} title={link.open}>
      <Link href="/notifications"><Bell /></Link>
    </Button>
  );
};
