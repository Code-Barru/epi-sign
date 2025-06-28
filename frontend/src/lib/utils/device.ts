export function isMobileDevice(): boolean {
    // if (typeof window === 'undefined') return false;
    
    // const userAgent = window.navigator.userAgent.toLowerCase();
    // const mobileKeywords = ['android', 'webos', 'iphone', 'ipad', 'ipod', 'blackberry', 'windows phone'];
    
    // // Vérifier le user agent
    // const isMobileUA = mobileKeywords.some(keyword => userAgent.includes(keyword));
    
    // // Vérifier la taille de l'écran
    // const isMobileScreen = window.innerWidth <= 768;
    
    // // Vérifier si l'appareil a un écran tactile
    // const hasTouchScreen = 'ontouchstart' in window || navigator.maxTouchPoints > 0;
    
    // return isMobileUA || (isMobileScreen && hasTouchScreen);
    return true;
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