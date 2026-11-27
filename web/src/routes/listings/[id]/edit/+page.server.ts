import { error, redirect } from '@sveltejs/kit';
import type { Detail } from '#lib/stays.js';
import { api, rethrowPage } from '#lib/server/api.js';
import { saveStay } from '#lib/server/stay-page.js';
import type { Actions, PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ parent, params, cookies }) => {
    const { user } = await parent();
    if (!user) redirect(303, `/login?next=/listings/${encodeURIComponent(params.id)}/edit`);
    const stay = await api<Detail>(`/listings/${encodeURIComponent(params.id)}`, cookies).catch(rethrowPage);
    if (stay.owner?.id !== user.id) error(403, 'Only the host can edit this stay.');
    return { stay };
};

export const actions: Actions = { default: (event) => saveStay(event, encodeURIComponent(event.params.id)) };
