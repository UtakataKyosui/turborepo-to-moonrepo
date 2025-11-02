// Fitbit API integration utilities with in-memory token management

// Type definitions for API error responses
export interface FitbitAuthErrorResponse {
    error: string;
    authRequired: boolean;
    reAuthUrl: string | null;
    requiresReauth?: boolean; // 後方互換性のため
}

export interface FitbitApiErrorResponse {
    error: string;
}

// Type definitions for Fitbit API responses
export interface FitbitHeartRateResponse {
    "activities-heart": Array<{
        dateTime: string;
        value: {
            customHeartRateZones: any[];
            heartRateZones: Array<{
                caloriesOut: number;
                max: number;
                min: number;
                minutes: number;
                name: string;
            }>;
            restingHeartRate: number;
        };
    }>;
    "activities-heart-intraday"?: {
        dataset: Array<{
            time: string;
            value: number;
        }>;
        datasetInterval: number;
        datasetType: string;
    };
}

export interface FitbitTokenResponse {
    access_token: string;
    expires_in: number;
    refresh_token: string;
    scope: string;
    token_type: string;
    user_id: string;
}

// Constants
export const FITBIT_API_BASE_URL = "https://api.fitbit.com";
export const FITBIT_OAUTH_URL = "https://api.fitbit.com/oauth2/token";

// In-memory token cache (server-side only)
// Note: これはサーバー再起動時にリセットされます
let tokenCache: {
    accessToken: string | null;
    refreshToken: string | null;
    expiresAt: number | null;
} = {
    accessToken: null,
    refreshToken: null,
    expiresAt: null,
};

// Helper function to build Fitbit API URLs
export function buildFitbitApiUrl(endpoint: string): string {
    return `${FITBIT_API_BASE_URL}${endpoint}`;
}

// Custom error class for authentication errors
export class FitbitAuthError extends Error {
    public readonly authRequired: boolean;
    public readonly reAuthUrl: string | null;

    constructor(message: string, options?: { authRequired?: boolean; reAuthUrl?: string }) {
        super(message);
        this.name = 'FitbitAuthError';
        this.authRequired = options?.authRequired ?? false;
        this.reAuthUrl = options?.reAuthUrl ?? null;
    }
}

// Token management functions (server-side only)
export async function refreshAccessToken(): Promise<string> {
    const clientId = process.env.FITBIT_CLIENT_ID;
    const refreshToken = tokenCache.refreshToken || process.env.FITBIT_REFRESH_TOKEN;

    if (!clientId || !refreshToken) {
        throw new FitbitAuthError("Missing Fitbit credentials - please reconnect your account");
    }

    const body = new URLSearchParams();
    body.append("grant_type", "refresh_token");
    body.append("refresh_token", refreshToken);
    body.append("client_id", clientId);

    try {
        const response = await fetch(FITBIT_OAUTH_URL, {
            method: "POST",
            headers: {
                "Content-Type": "application/x-www-form-urlencoded"
            },
            body: body.toString()
        });

        if (!response.ok) {
            const errorText = await response.text();
            console.error("Failed to refresh access token:", errorText);

            // Check if it's an invalid_grant error (refresh token expired/revoked)
            if (errorText.includes("invalid_grant") || errorText.includes("Refresh token invalid")) {
                // Clear the invalid tokens
                clearTokenCache();

                // Generate re-authentication URL
                const reAuthUrl = '/api/fitbit/auth';

                throw new FitbitAuthError(
                    "Your Fitbit session has expired. Please reconnect your account.",
                    { authRequired: true, reAuthUrl }
                );
            }

            throw new Error(`Failed to refresh access token: ${errorText}`);
        }

        const data: FitbitTokenResponse = await response.json();

        // Update in-memory cache
        tokenCache.accessToken = data.access_token;
        tokenCache.refreshToken = data.refresh_token;
        tokenCache.expiresAt = Date.now() + (data.expires_in * 1000);

        console.log("Access token refreshed successfully");

        return data.access_token;
    } catch (error) {
        // If it's already a FitbitAuthError, rethrow it
        if (error instanceof FitbitAuthError) {
            throw error;
        }
        // Clear tokens on any error and throw auth error with re-auth URL
        clearTokenCache();
        throw new FitbitAuthError(
            "Failed to refresh Fitbit token. Please reconnect your account.",
            { authRequired: true, reAuthUrl: '/api/fitbit/auth' }
        );
    }
}

export async function getAccessToken(): Promise<string> {
    // Check if we have a valid cached token
    if (tokenCache.accessToken && tokenCache.expiresAt && Date.now() < tokenCache.expiresAt - 60000) {
        return tokenCache.accessToken;
    }

    // Token is expired or doesn't exist, refresh it
    return await refreshAccessToken();
}

/**
 * Generic wrapper for Fitbit API calls with automatic token refresh and retry logic
 * @template T The expected response type
 * @param endpoint The Fitbit API endpoint (e.g., '/1/user/-/activities/heart/date/today/1d.json')
 * @param options Additional fetch options (headers will be merged with Authorization header)
 * @returns The parsed JSON response
 * @throws FitbitAuthError if re-authentication is required
 */
export async function callFitbitApi<T = any>(
    endpoint: string,
    options: RequestInit = {}
): Promise<T> {
    try {
        // Get access token (will refresh if needed)
        const accessToken = await getAccessToken();

        // Make the API call
        const response = await fetch(`${FITBIT_API_BASE_URL}${endpoint}`, {
            ...options,
            headers: {
                ...options.headers,
                'Authorization': `Bearer ${accessToken}`,
            },
        });

        // Check if unauthorized (token might have been revoked)
        if (response.status === 401) {
            console.warn('Received 401, attempting token refresh...');

            // Try refreshing the token once
            const newAccessToken = await refreshAccessToken();

            // Retry the request with new token
            const retryResponse = await fetch(`${FITBIT_API_BASE_URL}${endpoint}`, {
                ...options,
                headers: {
                    ...options.headers,
                    'Authorization': `Bearer ${newAccessToken}`,
                },
            });

            if (!retryResponse.ok) {
                const errorText = await retryResponse.text();
                throw new Error(`Fitbit API request failed after retry: ${retryResponse.status} ${errorText}`);
            }

            return retryResponse.json();
        }

        // Check for other errors
        if (!response.ok) {
            const errorText = await response.text();
            throw new Error(`Fitbit API request failed: ${response.status} ${errorText}`);
        }

        return response.json();
    } catch (error) {
        // If it's a FitbitAuthError (re-auth required), propagate it
        if (error instanceof FitbitAuthError) {
            throw error;
        }

        // For other errors, wrap them
        console.error('Fitbit API call error:', error);
        throw error;
    }
}

// API call functions
/**
 * Fetch heart rate data for today with intraday details
 * Automatically handles token refresh and re-authentication if needed
 * @throws FitbitAuthError if re-authentication is required (authRequired=true)
 */
export async function getHeartRate(): Promise<FitbitHeartRateResponse> {
    return callFitbitApi<FitbitHeartRateResponse>(
        '/1/user/-/activities/heart/date/today/1d/1sec.json'
    );
}

// Client-side utilities (for use in browser)
export async function fetchHeartRateFromApi(): Promise<FitbitHeartRateResponse> {
    const response = await fetch("/api/fitbit/heart-rate");

    if (!response.ok) {
        const error = await response.json();
        throw new Error(error.error || "Failed to fetch heart rate data");
    }

    return response.json();
}

// Utility to manually set tokens (useful for initialization)
export function setTokens(accessToken: string, refreshToken: string, expiresIn: number = 3600): void {
    tokenCache.accessToken = accessToken;
    tokenCache.refreshToken = refreshToken;
    tokenCache.expiresAt = Date.now() + (expiresIn * 1000);
}

// Utility to clear token cache
export function clearTokenCache(): void {
    tokenCache.accessToken = null;
    tokenCache.refreshToken = null;
    tokenCache.expiresAt = null;
}
