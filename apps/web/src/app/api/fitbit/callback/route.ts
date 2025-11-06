import { NextRequest, NextResponse } from "next/server";
import { cookies } from "next/headers";
import { setTokens, type FitbitTokenResponse } from "@workspace/fitbit/index";

/**
 * GET /api/fitbit/callback
 * OAuth 2.0 コールバックエンドポイント
 * Fitbitから認証コードを受け取り、アクセストークンと交換します
 */
export async function GET(request: NextRequest) {
    try {
        const searchParams = request.nextUrl.searchParams;
        const code = searchParams.get('code');
        const state = searchParams.get('state');
        const error = searchParams.get('error');

        // エラーチェック
        if (error) {
            console.error('OAuth error:', error);
            return NextResponse.redirect(
                `${request.nextUrl.origin}/body-metrics?error=auth_failed&message=${encodeURIComponent(error)}`
            );
        }

        if (!code) {
            return NextResponse.redirect(
                `${request.nextUrl.origin}/body-metrics?error=no_code`
            );
        }

        // Cookieから保存されたPKCEデータを取得
        const cookieStore = await cookies();
        const savedState = cookieStore.get('fitbit_pkce_state')?.value;
        const codeVerifier = cookieStore.get('fitbit_pkce_verifier')?.value;

        // State検証（CSRF対策）
        if (!savedState || state !== savedState) {
            console.error('State mismatch:', { savedState, receivedState: state });
            return NextResponse.redirect(
                `${request.nextUrl.origin}/body-metrics?error=invalid_state`
            );
        }

        if (!codeVerifier) {
            return NextResponse.redirect(
                `${request.nextUrl.origin}/body-metrics?error=no_verifier`
            );
        }

        // 環境変数からクライアント情報を取得
        const clientId = process.env.FITBIT_CLIENT_ID;
        if (!clientId) {
            throw new Error('FITBIT_CLIENT_ID not configured');
        }

        const redirectUri = process.env.FITBIT_REDIRECT_URI ||
            `${request.nextUrl.origin}/api/fitbit/callback`;

        // アクセストークンを取得
        const tokenResponse = await exchangeCodeForToken({
            code,
            codeVerifier,
            clientId,
            redirectUri,
        });

        // トークンをメモリキャッシュに保存
        setTokens(
            tokenResponse.access_token,
            tokenResponse.refresh_token,
            tokenResponse.expires_in
        );

        // PKCE Cookieをクリア
        cookieStore.delete('fitbit_pkce_state');
        cookieStore.delete('fitbit_pkce_verifier');

        // リダイレクト先を決定
        // Note: sessionStorageはサーバーサイドでアクセスできないため、
        // クライアントサイドで処理するか、デフォルトページを使用
        const defaultReturnPath = '/contents/body-metrics';

        // 成功ページにリダイレクト
        return NextResponse.redirect(
            `${request.nextUrl.origin}${defaultReturnPath}?success=true`
        );
    } catch (error) {
        console.error('Error in OAuth callback:', error);
        return NextResponse.redirect(
            `${request.nextUrl.origin}/body-metrics?error=token_exchange_failed`
        );
    }
}

/**
 * 認証コードをアクセストークンに交換
 */
async function exchangeCodeForToken(params: {
    code: string;
    codeVerifier: string;
    clientId: string;
    redirectUri: string;
}): Promise<FitbitTokenResponse> {
    const { code, codeVerifier, clientId, redirectUri } = params;

    const body = new URLSearchParams();
    body.append('client_id', clientId);
    body.append('code', code);
    body.append('code_verifier', codeVerifier);
    body.append('grant_type', 'authorization_code');
    body.append('redirect_uri', redirectUri);

    const response = await fetch('https://api.fitbit.com/oauth2/token', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/x-www-form-urlencoded',
        },
        body: body.toString(),
    });

    if (!response.ok) {
        const errorText = await response.text();
        console.error('Token exchange failed:', errorText);
        throw new Error(`Token exchange failed: ${response.status} ${errorText}`);
    }

    return response.json();
}
