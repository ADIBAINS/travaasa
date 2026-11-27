import { getUser } from '#lib/server/api.js';
import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ cookies, setHeaders }) => {
    const user = await getUser(cookies);
    if (user) setHeaders({ 'cache-control': 'private, no-store' });
    return { user };
};
