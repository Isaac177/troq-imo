import { Skeleton } from '@mungini/ui-web';

const SKELETON_ROWS = 5;

export default function NotificationListSkeleton() {
  return (
    <div aria-hidden="true" className="divide-y divide-border">
      {Array.from({ length: SKELETON_ROWS }, (_, index) => (
        <div key={index} className="flex flex-col gap-2 px-3 py-3 md:px-4">
          <Skeleton className="w-2/3" />
          <Skeleton className="w-full" />
          <Skeleton className="w-1/3" />
        </div>
      ))}
    </div>
  );
}
