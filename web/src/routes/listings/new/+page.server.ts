import { redirect } from '@sveltejs/kit';
import { saveStay } from '#lib/server/stay-page.js';
import type { Actions, PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ parent }) => {
    if (!(await parent()).user) redirect(303, '/login?next=/listings/new');
};

export const actions: Actions = { default: (event) => saveStay(event) };
