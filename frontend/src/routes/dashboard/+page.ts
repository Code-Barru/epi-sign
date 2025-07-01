import type { PageLoad } from './$types';
import { browser } from '$app/environment';
import { redirect } from '@sveltejs/kit';
import { loadUsers, checkAuth } from '$lib/api';
import { get } from 'svelte/store';
import { currentUser } from '$lib/stores';
import type { PublicUserResponse } from '$lib/types';

export const load: PageLoad = async ({ fetch }) => {
    // Vérification d'authentification uniquement côté client
    if (browser) {
        const authenticated = await checkAuth(fetch);
        
        if (!authenticated) {
            throw redirect(302, '/login');
        }

        try {
            const loadedUsers = await loadUsers(fetch);
            
            // Marquer les utilisateurs avec JWT expiré
            loadedUsers.forEach((user) => {
                user.jwtIsExpired =
                    user.jwtExpiresAt === undefined ||
                    user.jwtExpiresAt === null ||
                    new Date(user.jwtExpiresAt) < new Date();
            });

            // Trier les utilisateurs :
            // 1. Utilisateur actuel en premier
            // 2. Par validité de token (valides avant expirés)
            // 3. Par ULID
            const currentUserId = get(currentUser)?.id;
            const sortedUsers = loadedUsers.sort((a, b) => {
                // 1. Utilisateur actuel en premier
                const aIsCurrent = currentUserId === a.id;
                const bIsCurrent = currentUserId === b.id;

                if (aIsCurrent && !bIsCurrent) return -1;
                if (!aIsCurrent && bIsCurrent) return 1;

                // 2. Trier par validité de token (valides en premier)
                if (a.jwtIsExpired !== b.jwtIsExpired) {
                    return a.jwtIsExpired ? 1 : -1;
                }

                // 3. Trier par ULID
                return a.id.localeCompare(b.id);
            });

            return {
                users: sortedUsers as PublicUserResponse[]
            };
        } catch (error) {
            console.error('Erreur lors du chargement des utilisateurs:', error);
            return {
                users: [] as PublicUserResponse[],
                error: 'Impossible de charger les utilisateurs'
            };
        }
    }

    // Côté serveur, retourner un état initial
    return {
        users: [] as PublicUserResponse[]
    };
};