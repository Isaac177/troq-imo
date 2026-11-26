import createClient from 'openapi-fetch';
import type { ClientOptions } from 'openapi-fetch';
import type { paths } from './generated';

export type { paths, components, operations } from './generated';
export const createApiClient = (options: ClientOptions) => createClient<paths>(options);
