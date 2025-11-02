import { Suspense } from "react";
import { BodyMetricsContent } from "./body-metrics-content";
import type { Metadata } from "next/types";

export const metadata: Metadata = {
    title: "Body Metrics",
    description: "Track and visualize your body metrics over time.",
}

export default function BodyMetrics() {
    return (
        <Suspense fallback={
            <div className="container mx-auto p-6">
                <div className="flex flex-col items-center justify-center py-12">
                    <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-primary"></div>
                    <p className="mt-4 text-muted-foreground">Loading...</p>
                </div>
            </div>
        }>
            <BodyMetricsContent />
        </Suspense>
    )
}
