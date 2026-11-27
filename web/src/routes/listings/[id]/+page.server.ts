import { fail, redirect, type RequestEvent } from '@sveltejs/kit';
import type { Detail } from '#lib/stays.js';
import { api, ApiFailure, json, rethrowPage } from '#lib/server/api.js';
import type { Actions, PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ cookies, params, parent }) => {
    const path = `/listings/${encodeURIComponent(params.id)}`;
    const { user } = await parent();
    const [stay, saved] = await Promise.all([
        api<Detail>(path, cookies), user ? api<boolean>(`${path}/saved`, cookies) : false
    ]).catch(rethrowPage);
    const token = process.env.PUBLIC_MAPBOX_TOKEN;
    const point = stay.coordinates?.join(',');
    const mapImage = point && token?.startsWith('pk.')
        ? `https://api.mapbox.com/styles/v1/mapbox/streets-v12/static/pin-s+e4434e(${point})/${point},10/800x350?access_token=${encodeURIComponent(token)}`
        : null;
    return { stay, saved, mapImage };
};

type Operation = 'save' | 'unsave' | 'review' | 'deleteReview' | 'deleteStay';

function action(operation: Operation) {
    return async ({ cookies, params, request }: RequestEvent<{ id: string }>) => {
        const path = `/listings/${encodeURIComponent(params.id)}`;
        const form = await request.formData();
        try {
            switch (operation) {
                case 'save':
                case 'unsave':
                    await api(`${path}/saved`, cookies, { method: operation === 'save' ? 'PUT' : 'DELETE' });
                    break;
                case 'review':
                    await api(`${path}/reviews`, cookies, {
                        method: 'POST', ...json({ rating: Number(form.get('rating')), comment: form.get('comment') })
                    });
                    break;
                case 'deleteReview':
                    await api(`${path}/reviews/${encodeURIComponent(String(form.get('reviewId') ?? ''))}`, cookies, { method: 'DELETE' });
                    break;
                case 'deleteStay':
                    if (form.get('confirm') !== 'yes') return fail(400, { message: 'Confirm that you want to delete this stay.' });
                    await api(path, cookies, { method: 'DELETE' });
                    break;
            }
        } catch (exception) {
            if (exception instanceof ApiFailure) {
                if (exception.status === 401) redirect(303, `/login?next=${path}`);
                return fail(exception.status, { message: exception.message });
            }
            throw exception;
        }
        redirect(303, operation === 'deleteStay' ? '/listings' : path);
    };
}

export const actions: Actions = {
    save: action('save'), unsave: action('unsave'), review: action('review'),
    deleteReview: action('deleteReview'), deleteStay: action('deleteStay')
};
