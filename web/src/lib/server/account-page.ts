import { fail, redirect, type RequestEvent } from '@sveltejs/kit';
import { ApiFailure, authenticate } from './api.js';

export function destination(url: URL) {
    const next = url.searchParams.get('next');
    if (next && /^\/listings(?:\/(?:new|saved|[a-f0-9]{24}(?:\/edit)?))?$/.test(next)) return next;
    return '/listings';
}

export function accountAction(mode: 'login' | 'signup') {
    return async (event: RequestEvent) => {
        try {
            await authenticate(mode, event);
        } catch (exception) {
            if (exception instanceof ApiFailure) return fail(exception.status, { message: exception.message });
            throw exception;
        }
        redirect(303, destination(event.url));
    };
}
