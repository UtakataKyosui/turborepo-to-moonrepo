// PKCE (Proof Key for Code Exchange) utilities for Fitbit OAuth 2.0
// Based on RFC 7636: https://tools.ietf.org/html/rfc7636

/**
 * Generate a cryptographically secure random code verifier
 * Length: 43-128 characters (Fitbit requirement)
 * Characters: A-Z, a-z, 0-9, -, ., _, ~
 */
export function generateCodeVerifier(length: number = 128): string {
    const possible = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-._~';
    const values = crypto.getRandomValues(new Uint8Array(length));
    return Array.from(values)
        .map((x) => possible[x % possible.length])
        .join('');
}

/**
 * Generate a code challenge from a code verifier
 * Method: S256 (SHA-256 hash, base64url encoded)
 */
export async function generateCodeChallenge(verifier: string): Promise<string> {
    // Convert string to bytes
    const encoder = new TextEncoder();
    const data = encoder.encode(verifier);

    // SHA-256 hash
    const hashBuffer = await crypto.subtle.digest('SHA-256', data);

    // Convert to base64url (without padding)
    return base64UrlEncode(hashBuffer);
}

/**
 * Base64url encode (RFC 4648 Section 5)
 * - Convert to base64
 * - Replace + with -
 * - Replace / with _
 * - Remove padding (=)
 */
function base64UrlEncode(buffer: ArrayBuffer): string {
    const bytes = new Uint8Array(buffer);
    let binary = '';
    for (let i = 0; i < bytes.byteLength; i++) {
        const byte = bytes[i];
        if (byte !== undefined) {
            binary += String.fromCharCode(byte);
        }
    }

    return btoa(binary)
        .replace(/\+/g, '-')
        .replace(/\//g, '_')
        .replace(/=/g, '');
}

/**
 * Generate a random state value for CSRF protection
 * Length: 32 characters
 */
export function generateState(): string {
    return generateCodeVerifier(32);
}

/**
 * PKCE session data to be stored (e.g., in sessionStorage or database)
 */
export interface PKCESession {
    codeVerifier: string;
    codeChallenge: string;
    state: string;
    createdAt: number;
}

/**
 * Create a complete PKCE session
 */
export async function createPKCESession(): Promise<PKCESession> {
    const codeVerifier = generateCodeVerifier();
    const codeChallenge = await generateCodeChallenge(codeVerifier);
    const state = generateState();

    return {
        codeVerifier,
        codeChallenge,
        state,
        createdAt: Date.now(),
    };
}
