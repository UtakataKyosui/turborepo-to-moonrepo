import { NextResponse } from "next/server";
import { getHeartRate, FitbitAuthError } from "@workspace/fitbit/index";

/**
 * GET /api/fitbit/heart-rate
 * Fitbitの心拍数データを取得するAPI
 */
export async function GET() {
    try {
        const data = await getHeartRate();
        return NextResponse.json(data);
    } catch (error) {
        console.error("Error fetching heart rate:", error);

        // 認証エラーの場合は401を返す
        if (error instanceof FitbitAuthError) {
            return NextResponse.json(
                {
                    error: error.message,
                    authRequired: error.authRequired,
                    reAuthUrl: error.reAuthUrl,
                    requiresReauth: error.authRequired, // 後方互換性のため
                },
                { status: 401 }
            );
        }

        return NextResponse.json(
            { error: error instanceof Error ? error.message : "Failed to fetch heart rate data" },
            { status: 500 }
        );
    }
}
