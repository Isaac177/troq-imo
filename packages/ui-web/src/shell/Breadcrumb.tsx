import type { ReactNode } from 'react';

type BreadcrumbItem = {
  key: string;
  content: ReactNode;
  current?: boolean;
};

type BreadcrumbProps = {
  label: string;
  items: BreadcrumbItem[];
};

export function Breadcrumb({ label, items }: BreadcrumbProps) {
  return (
    <nav aria-label={label} className="mungini-breadcrumb">
      <ol>
        {items.map((item) => (
          <li key={item.key} aria-current={item.current ? 'page' : undefined}>
            {item.content}
          </li>
        ))}
      </ol>
    </nav>
  );
}
