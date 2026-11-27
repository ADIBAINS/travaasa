import { comparisonIds } from '#lib/compare.js';
import { api, ApiFailure, rethrowPage } from '#lib/server/api.js';
import type { Detail } from '#lib/stays.js';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ cookies, url }) => {
    const ids = comparisonIds(url.searchParams.getAll('stay'));
    const results = await Promise.all(ids.map(async (id) => {
        try {
            return await api<Detail>(`/listings/${id}`, cookies);
        } catch (exception) {
            if (exception instanceof ApiFailure && exception.status === 404) return null;
            rethrowPage(exception);
        }
    }));

    const stays = results.filter((stay) => stay !== null);
    return { stays, unavailable: ids.length - stays.length };
};
