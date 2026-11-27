import { error, fail, redirect, type Cookies, type RequestEvent } from '@sveltejs/kit';
import type { User, Catalog } from '#lib/stays.js';

const sessionCookie = 'travaasa_session';

export class ApiFailure extends Error {
    constructor(public status: number, message: string) {
        super(message);
    }
}

export function api<T = void>(path: string, cookies: Cookies, init?: RequestInit): Promise<T>;
export async function api(path: string, cookies: Cookies, init: RequestInit = {}): Promise<unknown> {
    const headers = new Headers(init.headers);
    const token = cookies.get(sessionCookie);
    if (token) headers.set('Authorization', `Bearer ${token}`);
    let response: Response;
    try {
        response = await fetch(new URL(`/api${path}`, process.env.API_URL ?? 'http://127.0.0.1:3001'), {
            ...init,
            headers,
            signal: AbortSignal.timeout(45_000),
            redirect: 'error'
        });
    } catch {
        throw new ApiFailure(503, 'Travaasa is temporarily unavailable. Please try again.');
    }
    if (!response.ok) {
        const body = await response.json().catch(() => null);
        const message = body?.message ?? (response.status < 500
            ? 'Check the form fields and try again.'
            : 'Travaasa is temporarily unavailable. Please try again.');
        throw new ApiFailure(response.status, message);
    }
    return response.headers.get('content-type')?.includes('application/json')
        ? await response.json()
        : undefined;
}

export const json = (body: unknown) => ({
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body)
});

export function rethrowPage(caught: unknown): never {
    if (caught instanceof ApiFailure) error(caught.status, caught.message);
    throw caught;
}

export async function getUser(cookies: Cookies): Promise<User | null> {
    if (!cookies.get(sessionCookie)) return null;
    try {
        return await api<User>('/auth/me', cookies);
    } catch (exception) {
        if (!(exception instanceof ApiFailure) || exception.status !== 401) rethrowPage(exception);
        cookies.delete(sessionCookie, { path: '/' });
        return null;
    }
}

export async function authenticate(mode: 'login' | 'signup', event: RequestEvent) {
    const form = await event.request.formData();
    const { username, email, password } = Object.fromEntries(form);
    const result = await api<{ token: string; expires_in: number }>(`/auth/${mode}`, event.cookies, {
        method: 'POST', ...json({ username, email, password })
    });
    event.cookies.set(sessionCookie, result.token, {
        path: '/', httpOnly: true, sameSite: 'lax', secure: event.url.protocol === 'https:',
        maxAge: result.expires_in
    });
}

export async function signOut(cookies: Cookies) {
    await api('/auth/logout', cookies, { method: 'POST' });
    cookies.delete(sessionCookie, { path: '/' });
}

export async function loadCatalog(event: RequestEvent, mode: 'explore' | 'saved' | 'mine', user: User | null) {
    const base = mode === 'explore' ? '/listings' : `/listings/${mode}`;
    if (mode !== 'explore' && !user) redirect(303, `/login?next=${base}`);
    const filters = Object.fromEntries(
        ['q', 'category', 'min', 'max', 'sort', 'page'].map((key) => [key, event.url.searchParams.get(key) ?? ''])
    );
    const query = new URLSearchParams(Object.entries(filters).filter(([, value]) => value !== ''));
    try {
        const [catalog, ids] = await Promise.all([
            api<Catalog>(`${base}?${query}`, event.cookies),
            user && mode === 'explore' ? api<string[]>('/listings/saved/ids', event.cookies) : Promise.resolve<string[]>([])
        ]);
        const savedIds = catalog.stays.filter((stay) => mode === 'saved' || ids.includes(stay.id)).map((stay) => stay.id);
        return { catalog, filters, base, mode, savedIds, message: '' };
    } catch (exception) {
        if (exception instanceof ApiFailure && exception.status === 400) {
            return { catalog: { stays: [], page: 1, pages: 1, count: 0 }, filters, base, mode, savedIds: [], message: exception.message };
        }
        rethrowPage(exception);
    }
}

export const catalogActions = {
    toggleSaved: async ({ request, cookies, url }: RequestEvent) => {
        const form = await request.formData();
        const id = form.get('id');
        const action = form.get('operation');
        if (typeof id !== 'string' || !/^[a-f0-9]{24}$/i.test(id) || (action !== 'save' && action !== 'unsave')) {
            return fail(400, { message: 'Choose a valid stay to save or remove.' });
        }

        try {
            await api(`/listings/${id}/saved`, cookies, { method: action === 'save' ? 'PUT' : 'DELETE' });
        } catch (exception) {
            if (exception instanceof ApiFailure) {
                if (exception.status === 401) redirect(303, `/login?next=${url.pathname}`);
                return fail(exception.status, { message: exception.message });
            }
            throw exception;
        }
        return { message: '', savedChange: action === 'save' ? 'Stay saved.' : 'Stay removed.' };
    }
};

export async function stayForm(request: Request) {
    const submitted = await request.formData();
    const fields = Object.fromEntries(
        ['title', 'description', 'location', 'country', 'category', 'price'].map((key) => [key, String(submitted.get(key) ?? '')])
    );
    const payload = new FormData();
    payload.set('stay', JSON.stringify({ ...fields, price: fields.price.trim() ? Number(fields.price) : null }));
    const image = submitted.get('image');
    if (image instanceof File && image.size > 0) payload.set('image', image);
    return { fields, payload };
}
