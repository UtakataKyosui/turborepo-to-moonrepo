"use client"

import { useState, useEffect, useCallback } from "react";
import { useSearchParams } from "next/navigation";
import type { FitbitHeartRateResponse, FitbitAuthErrorResponse } from "@workspace/fitbit/index";

interface UseFitbitHeartRateReturn {
    heartRateData: FitbitHeartRateResponse | null;
    loading: boolean;
    error: string | null;
    isRealTime: boolean;
    lastUpdate: Date | null;
    isAuthenticated: boolean;
    authMessage: { type: 'success' | 'error', message: string } | null;
    currentBpm: number;
    restingHeartRate: number | undefined;
    setIsRealTime: (value: boolean) => void;
    fetchData: () => Promise<void>;
    handleConnectFitbit: () => void;
}

export function useFitbitHeartRate(): UseFitbitHeartRateReturn {
    const searchParams = useSearchParams();
    const [heartRateData, setHeartRateData] = useState<FitbitHeartRateResponse | null>(null);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);
    const [isRealTime, setIsRealTime] = useState(false);
    const [lastUpdate, setLastUpdate] = useState<Date | null>(null);
    const [isAuthenticated, setIsAuthenticated] = useState(false);
    const [authMessage, setAuthMessage] = useState<{ type: 'success' | 'error', message: string } | null>(null);

    // OAuth コールバックの処理
    useEffect(() => {
        const success = searchParams.get('success');
        const errorParam = searchParams.get('error');
        const message = searchParams.get('message');

        if (success === 'true') {
            setAuthMessage({ type: 'success', message: 'Successfully connected to Fitbit!' });
            setIsAuthenticated(true);
            // URLパラメータをクリア
            window.history.replaceState({}, '', window.location.pathname);
        } else if (errorParam) {
            const errorMessages: Record<string, string> = {
                'auth_failed': 'Authentication failed',
                'no_code': 'No authorization code received',
                'invalid_state': 'Invalid state parameter (security check failed)',
                'no_verifier': 'PKCE verifier not found',
                'token_exchange_failed': 'Failed to exchange token'
            };
            setAuthMessage({
                type: 'error',
                message: message ? decodeURIComponent(message) : errorMessages[errorParam] || 'Unknown error occurred'
            });
            // URLパラメータをクリア
            window.history.replaceState({}, '', window.location.pathname);
        }
    }, [searchParams]);

    const fetchData = useCallback(async () => {
        try {
            setError(null);
            const response = await fetch("/api/fitbit/heart-rate");

            // 認証エラーの場合
            if (response.status === 401) {
                const errorData: FitbitAuthErrorResponse = await response.json();
                setIsAuthenticated(false);

                // 自動再認証が必要な場合
                if (errorData.authRequired && errorData.reAuthUrl) {
                    console.log("Token expired, redirecting to re-authenticate...");
                    // 現在のURLを保存（認証後に戻るため）
                    sessionStorage.setItem('returnUrl', window.location.pathname);
                    // 再認証ページにリダイレクト
                    window.location.href = errorData.reAuthUrl;
                    return;
                }

                setError(errorData.error || "Your Fitbit session has expired. Please reconnect your account.");
                setLoading(false);
                return;
            }

            if (!response.ok) {
                const errorData = await response.json();
                throw new Error(errorData.error || "Failed to fetch heart rate data");
            }

            const data = await response.json();
            console.log("Heart Rate Data:", data);
            setHeartRateData(data);
            setLastUpdate(new Date());
            setIsAuthenticated(true);
        } catch (err) {
            console.error("Error fetching heart rate:", err);
            const errorMessage = err instanceof Error ? err.message : "Failed to fetch heart rate data";
            setError(errorMessage);

            // 認証エラーの場合は未認証状態にする
            if (errorMessage.includes('reconnect') || errorMessage.includes('expired') || errorMessage.includes('session')) {
                setIsAuthenticated(false);
            }
        } finally {
            setLoading(false);
        }
    }, []);

    // 初回データ取得
    useEffect(() => {
        fetchData();
    }, [fetchData]);

    // リアルタイム更新（5秒ごと）
    useEffect(() => {
        if (!isRealTime) return;

        const interval = setInterval(() => {
            fetchData();
        }, 5000); // 5秒ごとに更新

        return () => clearInterval(interval);
    }, [isRealTime, fetchData]);

    // 心拍数を取得（安静時心拍数またはイントラデイデータの最新値）
    const getCurrentHeartRate = (): number => {
        // イントラデイデータがあれば最新の値を使用
        const intradayData = heartRateData?.["activities-heart-intraday"]?.dataset;
        if (intradayData && intradayData.length > 0) {
            const lastValue = intradayData[intradayData.length - 1];
            return lastValue?.value ?? 0;
        }

        // なければ安静時心拍数を使用
        return heartRateData?.["activities-heart"]?.[0]?.value?.restingHeartRate ?? 0;
    };

    const currentBpm = getCurrentHeartRate();
    const restingHeartRate = heartRateData?.["activities-heart"]?.[0]?.value?.restingHeartRate;

    // Fitbit認証を開始
    const handleConnectFitbit = () => {
        window.location.href = '/api/fitbit/auth';
    };

    return {
        heartRateData,
        loading,
        error,
        isRealTime,
        lastUpdate,
        isAuthenticated,
        authMessage,
        currentBpm,
        restingHeartRate,
        setIsRealTime,
        fetchData,
        handleConnectFitbit
    };
}
