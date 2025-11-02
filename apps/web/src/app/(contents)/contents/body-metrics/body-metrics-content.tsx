"use client"

import { HeartBeatAnimation } from "@/components/heart-beat-animation";
import { useFitbitHeartRate } from "@/hooks/useFitbitHeartRate";

export function BodyMetricsContent() {
    const {
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
    } = useFitbitHeartRate();

    return (
        <div className="container mx-auto p-6 space-y-8">
            <div className="flex items-center justify-between">
                <h1 className="text-3xl font-bold">Body Metrics</h1>

                {/* 認証状態に応じた表示 */}
                {isAuthenticated ? (
                    <div className="flex flex-col items-end gap-3">
                        <div className="flex items-center gap-3">
                            <label className="flex items-center gap-2 cursor-pointer">
                                <input
                                    type="checkbox"
                                    checked={isRealTime}
                                    onChange={(e) => setIsRealTime(e.target.checked)}
                                    className="w-4 h-4"
                                />
                                <span className="text-sm">
                                    Real-time updates (5s)
                                </span>
                            </label>

                            <button
                                onClick={fetchData}
                                disabled={loading}
                                className="px-4 py-2 bg-primary text-primary-foreground rounded-md hover:bg-primary/90 disabled:opacity-50 transition-colors text-sm"
                            >
                                {loading ? "Updating..." : "Refresh"}
                            </button>
                        </div>

                        {isRealTime && (
                            <p className="text-xs text-muted-foreground max-w-md text-right">
                                ※ Fitbitデバイスからのデータ同期は数分ごとに行われます。
                                最新データを確認するには、Fitbitアプリで手動同期してください。
                            </p>
                        )}
                    </div>
                ) : (
                    <button
                        onClick={handleConnectFitbit}
                        className="px-6 py-3 bg-[#00B0B9] text-white rounded-md hover:bg-[#009DA5] transition-colors font-medium"
                    >
                        Connect Fitbit
                    </button>
                )}
            </div>

            {/* 認証メッセージ表示 */}
            {authMessage && (
                <div className={`p-4 rounded-lg border ${
                    authMessage.type === 'success'
                        ? 'bg-green-50 border-green-200 text-green-800'
                        : 'bg-destructive/10 border-destructive/20 text-destructive'
                }`}>
                    <p className="font-medium">{authMessage.message}</p>
                </div>
            )}

            {/* 最終更新時刻と同期情報 */}
            {lastUpdate && (
                <div className="flex items-center justify-between bg-muted/30 p-3 rounded-lg">
                    <div className="text-xs text-muted-foreground">
                        Last updated: {lastUpdate.toLocaleTimeString()}
                        {isRealTime && <span className="ml-2 text-green-500">● Live</span>}
                    </div>
                    <div className="text-xs text-muted-foreground flex items-center gap-2">
                        <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                        </svg>
                        <span>Fitbitアプリで同期するとデータが更新されます</span>
                    </div>
                </div>
            )}

            {/* 未認証状態の表示 */}
            {!isAuthenticated && !loading && (
                <div className="flex flex-col items-center justify-center py-16 space-y-6">
                    <div className="text-center space-y-2">
                        <h2 className="text-2xl font-semibold">Connect to Fitbit</h2>
                        <p className="text-muted-foreground max-w-md">
                            Connect your Fitbit account to view your heart rate data and other body metrics in real-time.
                        </p>
                    </div>
                    <button
                        onClick={handleConnectFitbit}
                        className="px-8 py-4 bg-[#00B0B9] text-white rounded-lg hover:bg-[#009DA5] transition-colors font-medium text-lg"
                    >
                        Connect Fitbit Account
                    </button>
                    <p className="text-xs text-muted-foreground">
                        You will be redirected to Fitbit to authorize this application
                    </p>
                </div>
            )}

            {/* エラー表示 */}
            {error && isAuthenticated && (
                <div className="p-4 bg-destructive/10 border border-destructive/20 rounded-lg">
                    <p className="text-destructive font-medium">Error: {error}</p>
                    <p className="text-sm text-muted-foreground mt-2">
                        Try reconnecting your Fitbit account if the problem persists.
                    </p>
                </div>
            )}

            {/* ハートビートアニメーション */}
            {isAuthenticated && !loading && !error && currentBpm > 0 && (
                <div className="flex justify-center py-8">
                    <HeartBeatAnimation bpm={currentBpm} size={160} />
                </div>
            )}

            {/* データなしの案内 */}
            {isAuthenticated && !loading && !error && heartRateData && currentBpm === 0 && (
                <div className="p-6 bg-muted/50 border border-dashed rounded-lg">
                    <div className="flex items-start gap-3">
                        <svg className="w-6 h-6 text-muted-foreground mt-1 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                        </svg>
                        <div className="space-y-3">
                            <p className="font-medium text-foreground">
                                リアルタイム心拍数データがまだ取得されていません
                            </p>
                            <div className="text-sm text-muted-foreground space-y-2">
                                <p>データを取得するには：</p>
                                <ol className="list-decimal list-inside space-y-1 ml-2">
                                    <li>Fitbitデバイスを装着してください</li>
                                    <li>Fitbitアプリを開いて手動同期を実行してください</li>
                                    <li>数分待ってから、このページで「Refresh」ボタンをクリックしてください</li>
                                </ol>
                                <p className="mt-3 text-xs">
                                    ※ Fitbitのイントラデイデータは、デバイスとアプリの同期後に利用可能になります。
                                    同期頻度はデバイスの種類やBluetooth接続状態により異なります。
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            )}

            {/* 心拍数の詳細情報 */}
            {isAuthenticated && !loading && !error && heartRateData && (
                <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                    {/* 安静時心拍数 */}
                    <div className="p-6 bg-card border rounded-lg">
                        <h3 className="text-sm font-medium text-muted-foreground mb-2">
                            Resting Heart Rate
                        </h3>
                        <p className="text-3xl font-bold">
                            {restingHeartRate ?? "N/A"}
                            {restingHeartRate && <span className="text-lg text-muted-foreground ml-2">bpm</span>}
                        </p>
                        {!restingHeartRate && (
                            <p className="text-xs text-muted-foreground mt-2">
                                安静時心拍数は、デバイスを装着して睡眠後に計測されます
                            </p>
                        )}
                    </div>

                    {/* 心拍数ゾーン */}
                    {heartRateData["activities-heart"]?.[0]?.value?.heartRateZones && (
                        <div className="p-6 bg-card border rounded-lg">
                            <h3 className="text-sm font-medium text-muted-foreground mb-3">
                                Heart Rate Zones
                            </h3>
                            <div className="space-y-2">
                                {heartRateData["activities-heart"][0].value.heartRateZones.map((zone, index) => (
                                    <div key={index} className="flex justify-between text-sm">
                                        <span className="font-medium">{zone.name}</span>
                                        <span className="text-muted-foreground">
                                            {zone.min}-{zone.max} bpm ({zone.minutes} min)
                                        </span>
                                    </div>
                                ))}
                            </div>
                        </div>
                    )}
                </div>
            )}

            {/* デバッグ用：生データ表示 */}
            {isAuthenticated && !loading && !error && heartRateData && (
                <details className="mt-6">
                    <summary className="cursor-pointer text-sm text-muted-foreground hover:text-foreground">
                        Show raw data
                    </summary>
                    <pre className="mt-4 p-4 bg-muted rounded text-xs overflow-auto max-h-96">
                        {JSON.stringify(heartRateData, null, 2)}
                    </pre>
                </details>
            )}

            {/* ローディング状態 */}
            {loading && !heartRateData && isAuthenticated && (
                <div className="flex flex-col items-center justify-center py-12">
                    <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-primary"></div>
                    <p className="mt-4 text-muted-foreground">Loading heart rate data...</p>
                </div>
            )}
        </div>
    )
}
