export function isMobileDevice(): boolean {
    if (typeof window === 'undefined') return false;
    
    const userAgent = window.navigator.userAgent.toLowerCase();
    const mobileKeywords = ['android', 'webos', 'iphone', 'ipad', 'ipod', 'blackberry', 'windows phone'];
    
    const isMobileUA = mobileKeywords.some(keyword => userAgent.includes(keyword));
    
    const isMobileScreen = window.innerWidth <= 768;
    
    const hasTouchScreen = 'ontouchstart' in window || navigator.maxTouchPoints > 0;
    
    return isMobileUA || (isMobileScreen && hasTouchScreen);
}

export async function checkCameraPermission(): Promise<boolean> {
    try {
        const result = await navigator.permissions.query({ name: 'camera' as PermissionName });
        return result.state === 'granted';
    } catch {
        // Si l'API Permissions n'est pas supportée, on retourne true pour essayer
        return true;
    }
}