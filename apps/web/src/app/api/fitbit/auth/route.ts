import { NextRequest, NextResponse } from "next/server";
import { createPKCESession } from "@workspace/fitbit/pkce";
import { cookies } from "next/headers";

/**
 * GET /api/fitbit/auth
 * OAuth 2.0 authorization flow開始エンドポイント
 * ユーザーをFitbit認証ページにリダイレクトします
 */
export async function GET(request: NextRequest) {
    try {
        const clientId = process.env.FITBIT_CLIENT_ID;

        if (!clientId) {
            return NextResponse.json(
                { error: "FITBIT_CLIENT_ID not configured" },
                { status: 500 }
            );
        }

        // PKCE セッションを作成
        const pkceSession = await createPKCESession();

        // PKCEデータをHTTP-onlyクッキーに保存
        const cookieStore = await cookies();
        cookieStore.set('fitbit_pkce_verifier', pkceSession.codeVerifier, {
            httpOnly: true,
            secure: process.env.NODE_ENV === 'production',
            sameSite: 'lax',
            maxAge: 60 * 10, // 10分間有効
            path: '/',
        });

        cookieStore.set('fitbit_pkce_state', pkceSession.state, {
            httpOnly: true,
            secure: process.env.NODE_ENV === 'production',
            sameSite: 'lax',
            maxAge: 60 * 10, // 10分間有効
            path: '/',
        });

        // リダイレクトURIを構築
        const redirectUri = process.env.FITBIT_REDIRECT_URI ||
            `${request.nextUrl.origin}/api/fitbit/callback`;

        // スコープを設定（必要に応じて調整）
        const scopes = [
            'activity',
            'heartrate',
            'profile',
            'settings',
        ];

        // Fitbit認証URLを構築
        const authUrl = new URL('https://www.fitbit.com/oauth2/authorize');
        authUrl.searchParams.set('client_id', clientId);
        authUrl.searchParams.set('response_type', 'code');
        authUrl.searchParams.set('code_challenge', pkceSession.codeChallenge);
        authUrl.searchParams.set('code_challenge_method', 'S256');
        authUrl.searchParams.set('scope', scopes.join(' '));
        authUrl.searchParams.set('state', pkceSession.state);
        authUrl.searchParams.set('redirect_uri', redirectUri);

        // Fitbit認証ページにリダイレクト
        return NextResponse.redirect(authUrl.toString());
    } catch (error) {
        console.error('Error starting OAuth flow:', error);
        return NextResponse.json(
            { error: 'Failed to start authorization' },
            { status: 500 }
        );
    }
}
