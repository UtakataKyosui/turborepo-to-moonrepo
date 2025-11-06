# Fitbit Integration Package

This package provides OAuth 2.0 PKCE authentication and API integration for Fitbit Web API.

## Features

- **OAuth 2.0 PKCE Flow**: Secure authorization using Proof Key for Code Exchange (RFC 7636)
- **Automatic Token Refresh**: Access tokens are automatically refreshed when expired
- **Automatic Re-authentication**: PKCE flow automatically restarts when refresh token is invalid
- **Unified Error Handling**: Consistent error handling across all API calls
- **In-Memory Token Management**: Server-side token storage with automatic refresh
- **Heart Rate API**: Fetch daily and intraday heart rate data
- **Type-Safe**: Full TypeScript support with type definitions
- **Generic API Wrapper**: Easy-to-use wrapper for any Fitbit API endpoint

## Setup

### 1. Register a Fitbit Application

1. Go to [Fitbit Developer Portal](https://dev.fitbit.com/apps)
2. Click "Register an App"
3. Fill in the application details:
   - **Application Name**: Your app name
   - **Description**: Brief description
   - **Application Website**: Your website URL
   - **Organization**: Your organization name
   - **OAuth 2.0 Application Type**: Select "Personal"
   - **Callback URL**: `http://localhost:3000/api/fitbit/callback` (for development)
   - **Default Access Type**: Read Only
4. Save the **Client ID** (you don't need the Client Secret for PKCE)

### 2. Configure Environment Variables

Create a `.env.local` file in the root of your web app:

```bash
FITBIT_CLIENT_ID=your_client_id_here
FITBIT_REDIRECT_URI=http://localhost:3000/api/fitbit/callback
```

For production, update `FITBIT_REDIRECT_URI` to your production domain.

## Architecture

### OAuth 2.0 PKCE Flow

```
1. User clicks "Connect Fitbit"
   ↓
2. Generate PKCE session (code_verifier, code_challenge, state)
   ↓
3. Store verifier and state in HTTP-only cookies (10 min expiry)
   ↓
4. Redirect to Fitbit authorization page
   ↓
5. User authorizes the application
   ↓
6. Fitbit redirects back with authorization code
   ↓
7. Validate state parameter (CSRF protection)
   ↓
8. Exchange code for access/refresh tokens using code_verifier
   ↓
9. Store tokens in server-side memory cache
   ↓
10. Clean up PKCE cookies
```

### Token Management

- **Access Token**: Stored in memory with expiration tracking
- **Refresh Token**: Automatically used to refresh expired access tokens
- **Security**: Tokens never exposed to client-side code

### API Routes

- **`/api/fitbit/auth`**: Initiates OAuth flow
- **`/api/fitbit/callback`**: Handles OAuth callback
- **`/api/fitbit/heart-rate`**: Fetches heart rate data

## Usage

### Server-Side (Next.js Route Handler)

```typescript
import { getHeartRate, FitbitAuthError, callFitbitApi } from '@workspace/fitbit';
import { NextResponse } from 'next/server';

export async function GET() {
  try {
    // Fetch heart rate data (automatically refreshes token if needed)
    const data = await getHeartRate();
    return NextResponse.json(data);
  } catch (error) {
    // Authentication error - re-authentication required
    if (error instanceof FitbitAuthError) {
      return NextResponse.json(
        {
          error: error.message,
          authRequired: error.authRequired,
          reAuthUrl: error.reAuthUrl,
        },
        { status: 401 }
      );
    }

    // Other errors
    return NextResponse.json(
      { error: 'Failed to fetch data' },
      { status: 500 }
    );
  }
}
```

### Custom API Calls

```typescript
import { callFitbitApi } from '@workspace/fitbit';

// Call any Fitbit API endpoint
const profile = await callFitbitApi('/1/user/-/profile.json');

// POST request example
const response = await callFitbitApi('/1/user/-/activities/log.json', {
  method: 'POST',
  body: JSON.stringify({ ... }),
  headers: {
    'Content-Type': 'application/json',
  },
});
```

### Client-Side Integration with Auto Re-authentication

```typescript
'use client';

import { useState, useEffect } from 'react';
import type { FitbitAuthErrorResponse } from '@workspace/fitbit';

export function HeartRateDisplay() {
  const [data, setData] = useState(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    async function fetchData() {
      try {
        const response = await fetch('/api/fitbit/heart-rate');

        if (!response.ok) {
          if (response.status === 401) {
            const errorData: FitbitAuthErrorResponse = await response.json();

            // Auto-redirect to re-authentication if required
            if (errorData.authRequired && errorData.reAuthUrl) {
              // Save current URL to return after auth
              sessionStorage.setItem('returnUrl', window.location.pathname);
              window.location.href = errorData.reAuthUrl;
              return;
            }
          }

          const errorData = await response.json();
          setError(errorData.error);
          return;
        }

        const heartRateData = await response.json();
        setData(heartRateData);
      } catch (err) {
        setError('Network error');
      }
    }

    fetchData();
  }, []);

  if (error) return <div>Error: {error}</div>;
  if (!data) return <div>Loading...</div>;

  return <div>{/* Display data */}</div>;
}
```

### Authentication Flow

```typescript
// Redirect to Fitbit login
window.location.href = '/api/fitbit/auth';

// Handle callback (automatically handled by route)
// User will be redirected back to /body-metrics?success=true
```

## API Reference

### `callFitbitApi<T>(endpoint, options?)`

Generic wrapper for Fitbit API calls with automatic token refresh and retry logic.

**Parameters**:
- `endpoint`: string - API endpoint (e.g., `/1/user/-/profile.json`)
- `options`: RequestInit - Optional fetch options

**Returns**: `Promise<T>` - Parsed JSON response

**Throws**: `FitbitAuthError` - When re-authentication is required

**Example**:
```typescript
const profile = await callFitbitApi('/1/user/-/profile.json');
```

### `getHeartRate()`

Fetches heart rate data for today including:
- Resting heart rate
- Heart rate zones
- Intraday heart rate data (if available)

**Returns**: `Promise<FitbitHeartRateResponse>`

**Throws**: `FitbitAuthError` - When re-authentication is required

### `refreshAccessToken()`

Manually refresh the access token using the refresh token.

**Returns**: `Promise<string>` - New access token

**Throws**: `FitbitAuthError` - When refresh token is invalid (authRequired=true)

### `getAccessToken()`

Get the current access token, automatically refreshing if expired.

**Returns**: `Promise<string>` - Valid access token

**Throws**: `FitbitAuthError` - When re-authentication is required

### `setTokens(accessToken, refreshToken, expiresIn)`

Stores OAuth tokens in memory cache (used internally by callback handler).

**Parameters**:
- `accessToken`: string - OAuth access token
- `refreshToken`: string - OAuth refresh token
- `expiresIn`: number - Token expiration time in seconds (default: 3600)

### `clearTokenCache()`

Clear the token cache.

### `fetchHeartRateFromApi()`

Client-side helper to fetch heart rate data from the API route.

**Returns**: `Promise<FitbitHeartRateResponse>`

## Error Handling

### FitbitAuthError

Custom error class for authentication errors:

```typescript
class FitbitAuthError extends Error {
  authRequired: boolean;   // Whether re-authentication is needed
  reAuthUrl: string | null; // Re-authentication URL
}
```

### Error Flow

1. **Access token expired** → Automatically refreshed using refresh token
2. **Refresh token invalid** → Throws `FitbitAuthError` with `authRequired=true`
3. **API call fails (401)** → Retries once, then re-authenticates if needed
4. **Other errors** → Handled as normal errors

### Type Definitions

```typescript
interface FitbitAuthErrorResponse {
  error: string;
  authRequired: boolean;
  reAuthUrl: string | null;
  requiresReauth?: boolean; // For backward compatibility
}

interface FitbitApiErrorResponse {
  error: string;
}
```

## Security Features

- **PKCE**: No client secret required, more secure for public clients
- **State Parameter**: CSRF protection
- **HTTP-only Cookies**: PKCE data stored securely during auth flow
- **Server-Side Tokens**: Access tokens never exposed to client
- **Automatic Refresh**: Tokens refreshed before expiration

## Limitations

- **In-Memory Storage**: Tokens are lost on server restart
- **Single User**: Current implementation supports one authenticated user at a time
- **No Persistence**: For production, consider using a database for token storage
- **Data Sync Delay**: Fitbit intraday data updates depend on device-to-app sync frequency (typically every few minutes)
- **Real-time Updates**: The "real-time" feature polls the API every 30 seconds, but data freshness depends on Fitbit device sync

## Data Synchronization

### Understanding Fitbit Data Updates

Fitbit heart rate data updates follow this flow:

1. **Device Measurement**: Fitbit device continuously measures heart rate
2. **Device Storage**: Data stored locally on the device
3. **Bluetooth Sync**: Device syncs with Fitbit mobile app (automatic or manual)
4. **Cloud Upload**: Fitbit app uploads data to Fitbit servers
5. **API Availability**: Data becomes available via Fitbit Web API

**Important Notes**:
- Sync frequency varies by device model and Bluetooth connection
- Manual sync in Fitbit app provides fastest data updates
- API polling alone does not trigger device synchronization
- Intraday data may have delays of several minutes to hours

### Improving Data Freshness

To get the latest heart rate data:

1. **Manual Sync**: Open Fitbit mobile app and trigger manual sync
2. **Auto Sync**: Ensure Bluetooth is enabled and app is running in background
3. **Device Proximity**: Keep device close to phone for automatic sync
4. **Refresh Page**: Click "Refresh" button after syncing in Fitbit app

## Troubleshooting

### Token Becomes Invalid

The PKCE re-authentication flow will automatically start. Users will be redirected to `/api/fitbit/auth`.

### Environment Variables Not Set

If `FITBIT_CLIENT_ID` is not set, a 500 error will be returned.

### Debug Mode

Check server console for detailed logs:

```bash
# Example error logs
Failed to refresh access token: invalid_grant
Access token refreshed successfully
Received 401, attempting token refresh...
```

## Future Enhancements

- [ ] Database-backed token storage
- [ ] Multi-user support with user-specific tokens
- [ ] Additional Fitbit API endpoints (sleep, activity, nutrition)
- [ ] Webhook support for real-time updates
- [ ] Persistent session storage with return URL handling

## References

- [Fitbit Web API Documentation](https://dev.fitbit.com/build/reference/web-api/)
- [OAuth 2.0 PKCE - RFC 7636](https://tools.ietf.org/html/rfc7636)
- [Fitbit OAuth 2.0 Guide](https://dev.fitbit.com/build/reference/web-api/developer-guide/authorization/)
