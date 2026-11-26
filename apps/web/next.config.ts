import path from 'node:path';
import type { NextConfig } from 'next';

const nextConfig: NextConfig = {
  transpilePackages: ['@mungini/i18n', '@mungini/api', '@mungini/session', '@mungini/ui-web', '@mungini/utils', '@mungini/theme'],
  turbopack: {
    root: path.resolve(__dirname, '../..'),
  },
};

export default nextConfig;
