import { browser } from '$app/environment';
import { isAuthenticated, currentUser } from './stores';
import type { 
    LoginPayload, 
    RegisterPayload, 
    SignPayload, 
    SignResponse, 
    PublicUserResponse,
    ApiError,
    User, 
    JwtPayload
} from './types';

const API_BASE = '/api';

// Configuration pour les cookies
const fetchConfig: RequestInit = {
    credentials: 'include', // IMPORTANT: inclure les cookies
    headers: {
        'Content-Type': 'application/json',
    }
};

async function apiCall<T>(endpoint: string, options: RequestInit = {}): Promise<T> {
    const response = await fetch(`${API_BASE}${endpoint}`, {
        ...fetchConfig,
        ...options,
        headers: {
            ...fetchConfig.headers,
            ...options.headers
        }
    });

    if (!response.ok) {
        // Si 401, déconnecter l'utilisateur
        if (response.status === 401 && endpoint !== '/auth/login') {
            if (browser) {
                isAuthenticated.set(false);
                currentUser.set(null);
            }
        }
        
        const error: ApiError = {
            status: response.status,
            message: `HTTP error! status: ${response.status}`
        };
        throw error;
    }

    const contentType = response.headers.get('content-type');
    if (contentType && contentType.includes('application/json')) {
        return await response.json() as T;
    }
    return await response.text() as T;
}

export async function login(username: string, password: string): Promise<void> {
    const payload: LoginPayload = { username, password };
    
    await apiCall<void>('/auth/login', {
        method: 'POST',
        body: JSON.stringify(payload)
    });
    
    // Récupérer les infos utilisateur après connexion
    try {
        const user = await getCurrentUser();
        isAuthenticated.set(true);
        currentUser.set(user);
    } catch {
        // Si on ne peut pas récupérer l'utilisateur, on considère qu'on est connecté avec les infos minimales
        isAuthenticated.set(true);
        currentUser.set({ id: '', username });
    }
}

export async function register(username: string, password: string): Promise<void> {
    const payload: RegisterPayload = { username, password };
    
    await apiCall<void>('/auth/register', {
        method: 'POST',
        body: JSON.stringify(payload)
    });
}

export async function logout(): Promise<void> {
    try {
        await apiCall<void>('/auth/logout', { method: 'POST' });
    } finally {
        isAuthenticated.set(false);
        currentUser.set(null);
    }
}

export async function getCurrentUser(): Promise<User> {
    return await apiCall<User>('/users/me');
}

export async function loadUsers(): Promise<PublicUserResponse[]> {
    return await apiCall<PublicUserResponse[]>('/users');
}

export async function signUsers(ulids: string[], url: string): Promise<SignResponse> {
    const payload: SignPayload = { ulids, url };
    
    return await apiCall<SignResponse>('/sign', {
        method: 'POST',
        body: JSON.stringify(payload)
    });
}

// Fonction pour vérifier l'état de l'authentification
export async function checkAuth(): Promise<boolean> {
    try {
        const user = await getCurrentUser();
        isAuthenticated.set(true);
        currentUser.set(user);
        return true;
    } catch {
        isAuthenticated.set(false);
        currentUser.set(null);
        return false;
    }
}

export async function updateUserJWT(jwt: string): Promise<void> {
    const payload: JwtPayload = { jwt };
    
    await apiCall<void>('/users/me/update-jwt', {
        method: 'POST',
        body: JSON.stringify(payload)
    });
}