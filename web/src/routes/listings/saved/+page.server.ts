import { redirect } from '@sveltejs/kit';
import { loadCatalog } from '#lib/server/api.js';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async (event) => {
    if (!(await event.parent()).user) redirect(303, '/login?next=/listings/saved');
    return loadCatalog(event, true);
};
