"use client";

import { useEffect, useState } from "react";

interface HeartBeatAnimationProps {
    bpm: number; // Beats per minute
    size?: number;
    className?: string;
}

export function HeartBeatAnimation({ bpm, size = 120, className = "" }: HeartBeatAnimationProps) {
    // BPMから1拍の時間を計算（ミリ秒）
    const beatDuration = bpm > 0 ? (60 / bpm) * 1000 : 1000;

    return (
        <div className={`flex flex-col items-center justify-center ${className}`}>
            <div className="relative" style={{ width: size, height: size }}>
                {/* 心拍アニメーション用のハート */}
                <div
                    className="absolute inset-0 flex items-center justify-center"
                    style={{
                        animation: `heartbeat ${beatDuration}ms ease-in-out infinite`
                    }}
                >
                    <svg
                        viewBox="0 0 24 24"
                        fill="currentColor"
                        className="text-red-500"
                        style={{ width: size, height: size }}
                    >
                        <path d="M12 21.35l-1.45-1.32C5.4 15.36 2 12.28 2 8.5 2 5.42 4.42 3 7.5 3c1.74 0 3.41.81 4.5 2.09C13.09 3.81 14.76 3 16.5 3 19.58 3 22 5.42 22 8.5c0 3.78-3.4 6.86-8.55 11.54L12 21.35z" />
                    </svg>
                </div>

                {/* パルスエフェクト */}
                <div
                    className="absolute inset-0 flex items-center justify-center"
                    style={{
                        animation: `pulse-ring ${beatDuration}ms ease-out infinite`
                    }}
                >
                    <div
                        className="rounded-full border-4 border-red-500 opacity-0"
                        style={{ width: size, height: size }}
                    />
                </div>
            </div>

            {/* BPM表示 */}
            <div className="mt-4 text-center">
                <div className="text-4xl font-bold text-red-500 tabular-nums">
                    {bpm}
                </div>
                <div className="text-sm text-muted-foreground uppercase tracking-wider">
                    BPM
                </div>
            </div>

            {/* 心拍数のステータス */}
            <div className="mt-2 text-xs text-muted-foreground">
                {getHeartRateStatus(bpm)}
            </div>

            <style jsx>{`
                @keyframes heartbeat {
                    0%, 100% {
                        transform: scale(1);
                    }
                    10% {
                        transform: scale(1.1);
                    }
                    20% {
                        transform: scale(1);
                    }
                    30% {
                        transform: scale(1.1);
                    }
                    40% {
                        transform: scale(1);
                    }
                }

                @keyframes pulse-ring {
                    0% {
                        transform: scale(0.9);
                        opacity: 1;
                    }
                    100% {
                        transform: scale(1.5);
                        opacity: 0;
                    }
                }
            `}</style>
        </div>
    );
}

function getHeartRateStatus(bpm: number): string {
    if (bpm < 40) return "Very Low";
    if (bpm < 60) return "Low / Resting";
    if (bpm < 100) return "Normal";
    if (bpm < 120) return "Elevated";
    if (bpm < 140) return "High";
    return "Very High";
}
