export interface User {
    id: string;
    username: string;
    jwtExpiresAt?: string | null;
    jwtIntraEpitech?: string | null;
}

export interface PublicUserResponse {
    id: string;
    username: string;
    jwtExpiresAt?: string | null;
    jwtIsExpired?: boolean;
}

export interface LoginPayload {
    username: string;
    password: string;
}

export interface RegisterPayload {
    username: string;
    password: string;
}

export interface SignPayload {
    ulids: string[];
    url: string;
}

export type SignResponse = 
    | 'success'
    | 'tokenExpired'
    | 'tokenNotFound'
    | 'alreadySigned'
    | 'unknownError'
    | 'serviceUnavailable';

export interface JwtPayload {
    jwt: string;
}

export interface ApiError {
    status: number;
    message: string;
}