import { error, redirect } from '@sveltejs/kit';
import { rethrowPage, signOut } from '#lib/server/api.js';
import type { RequestHandler } from './$types';

export const POST: RequestHandler = async ({ request, cookies, url }) => {
    if (request.headers.get('origin') !== url.origin) error(403, 'Reload Travaasa before signing out.');
    try {
        await signOut(cookies);
    } catch (exception) {
        rethrowPage(exception);
    }
    redirect(303, '/listings');
};
