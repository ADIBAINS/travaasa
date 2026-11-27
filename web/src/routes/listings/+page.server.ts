import { loadCatalog } from '#lib/server/api.js';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = (event) => loadCatalog(event, false);
