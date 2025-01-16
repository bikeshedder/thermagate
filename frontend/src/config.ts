export const CONFIG = {
    "WEBSOCKET_URL": import.meta.env.VITE_WEBSOCKET_URL as string,
    "API_URL": import.meta.env.VITE_API_URL as string,
}

export function apiUrl(path: string) {
    return `${CONFIG.API_URL}/${path}`
}
