import { fail, redirect, type RequestEvent } from '@sveltejs/kit';
import { api, ApiFailure, getUser, stayForm } from './api.js';

export async function saveStay(event: RequestEvent, id?: string) {
    if (!await getUser(event.cookies)) redirect(303, '/login');
    const { fields, payload } = await stayForm(event.request);
    let target = id;
    try {
        if (id) {
            await api(`/listings/${id}`, event.cookies, { method: 'PUT', body: payload });
        } else {
            const created = await api<{ id: string }>('/listings', event.cookies, { method: 'POST', body: payload });
            target = created.id;
        }
    } catch (exception) {
        if (exception instanceof ApiFailure) return fail(exception.status, { message: exception.message, fields });
        throw exception;
    }
    redirect(303, `/listings/${target}`);
}
