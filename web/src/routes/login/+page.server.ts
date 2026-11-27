import { redirect } from '@sveltejs/kit';
import { accountAction, destination } from '#lib/server/account-page.js';
import type { Actions, PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ parent, url }) => {
    if ((await parent()).user) redirect(303, destination(url));
};

export const actions: Actions = { default: accountAction('login') };
