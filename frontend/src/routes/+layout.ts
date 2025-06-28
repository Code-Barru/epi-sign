import type { LayoutLoad } from './$types';
import { browser } from '$app/environment';
import { checkAuth } from '$lib/api';

export const prerender = true; // Désactiver SSR pour éviter les problèmes de cookies

export const load: LayoutLoad = async () => {
    // Vérifier l'authentification uniquement côté client
    if (browser) {
        await checkAuth();
    }
    
    return {};
};