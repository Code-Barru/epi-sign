import type { PageLoad } from './$types';
import { browser } from '$app/environment';
import { redirect } from '@sveltejs/kit';
import { checkAuth } from '$lib/api';

export const load: PageLoad = async ({ fetch }) => {
    // Vérification d'authentification uniquement côté client
    if (browser) {
        const authenticated = await checkAuth(fetch);
        
        if (authenticated) {
            throw redirect(302, '/');
        }
    }
};