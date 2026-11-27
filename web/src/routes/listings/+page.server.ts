import { catalogActions, loadCatalog } from '#lib/server/api.js';
import type { Actions, PageServerLoad } from './$types';

export const load: PageServerLoad = async (event) => loadCatalog(event, 'explore', (await event.parent()).user);
export const actions: Actions = catalogActions;
