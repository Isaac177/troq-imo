import { clsx, type ClassValue } from 'clsx';
import { extendTailwindMerge } from 'tailwind-merge';

const merge = extendTailwindMerge({
  extend: {
    classGroups: {
      'font-size': [{ text: ['micro', 'label', 'body', 'title'] }],
    },
  },
});

export const cn = (...inputs: ClassValue[]): string => merge(clsx(inputs));
