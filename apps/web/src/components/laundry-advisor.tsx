import {
  analyzeLaundryConditions,
  getRecommendationLabel,
  getRecommendationColor,
  getRecommendationIcon,
  type Forecast,
} from "@workspace/weather"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@workspace/ui/components/card"
import { Badge } from "@workspace/ui/components/badge"
import { AlertCircle, CheckCircle, Info } from "lucide-react"

interface LaundryAdvisorProps {
  forecast: Forecast
}

export function LaundryAdvisor({ forecast }: LaundryAdvisorProps) {
  const advice = analyzeLaundryConditions(forecast)
  const label = getRecommendationLabel(advice.recommendation)
  const colorClass = getRecommendationColor(advice.recommendation)
  const icon = getRecommendationIcon(advice.recommendation)

  return (
    <Card className="w-full">
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <span className="text-2xl">{icon}</span>
          洗濯物診断
        </CardTitle>
        <CardDescription>
          {forecast.dateLabel} ({forecast.date}) の洗濯物を干す条件
        </CardDescription>
      </CardHeader>
      <CardContent className="space-y-4">
        {/* 総合評価 */}
        <div className="flex items-center gap-4">
          <div className="flex-1">
            <div className="flex items-center justify-between mb-2">
              <span className="text-sm font-medium">総合評価</span>
              <Badge variant={advice.recommendation === "excellent" || advice.recommendation === "good" ? "default" : "destructive"}>
                {label}
              </Badge>
            </div>
            <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2.5">
              <div
                className={`h-2.5 rounded-full transition-all ${
                  advice.score >= 80
                    ? "bg-green-600"
                    : advice.score >= 60
                    ? "bg-blue-600"
                    : advice.score >= 40
                    ? "bg-yellow-600"
                    : advice.score >= 20
                    ? "bg-orange-600"
                    : "bg-red-600"
                }`}
                style={{ width: `${advice.score}%` }}
              />
            </div>
            <p className="text-xs text-gray-500 dark:text-gray-400 mt-1">
              スコア: {advice.score}/100
            </p>
          </div>
        </div>

        {/* 最適時間帯 */}
        {advice.bestTimeSlot && (
          <div className="flex items-start gap-2 p-3 bg-blue-50 dark:bg-blue-950 rounded-lg">
            <CheckCircle className="w-5 h-5 text-blue-600 dark:text-blue-400 mt-0.5 flex-shrink-0" />
            <div>
              <p className="text-sm font-medium text-blue-900 dark:text-blue-100">
                最適な洗濯時間帯
              </p>
              <p className="text-sm text-blue-700 dark:text-blue-300">
                {advice.bestTimeSlot}
              </p>
            </div>
          </div>
        )}

        {/* 理由 */}
        <div className="space-y-2">
          <h4 className="text-sm font-semibold flex items-center gap-2">
            <Info className="w-4 h-4" />
            診断理由
          </h4>
          <ul className="space-y-1.5">
            {advice.reasons.map((reason: string, index: number) => (
              <li key={index} className="text-sm text-gray-700 dark:text-gray-300 flex items-start gap-2">
                <span className="text-gray-400 dark:text-gray-500 mt-1">•</span>
                <span>{reason}</span>
              </li>
            ))}
          </ul>
        </div>

        {/* 警告 */}
        {advice.warnings.length > 0 && (
          <div className="space-y-2">
            <h4 className="text-sm font-semibold flex items-center gap-2 text-orange-700 dark:text-orange-400">
              <AlertCircle className="w-4 h-4" />
              注意事項
            </h4>
            <ul className="space-y-1.5">
              {advice.warnings.map((warning: string, index: number) => (
                <li
                  key={index}
                  className="text-sm text-orange-700 dark:text-orange-300 flex items-start gap-2"
                >
                  <span className="text-orange-400 dark:text-orange-500 mt-1">⚠️</span>
                  <span>{warning}</span>
                </li>
              ))}
            </ul>
          </div>
        )}

        {/* 時間帯別降水確率 */}
        <div className="space-y-2">
          <h4 className="text-sm font-semibold">時間帯別降水確率</h4>
          <div className="grid grid-cols-2 gap-2">
            <div className="p-2 bg-gray-50 dark:bg-gray-800 rounded">
              <p className="text-xs text-gray-500 dark:text-gray-400">00:00-06:00</p>
              <p className="text-sm font-medium">{forecast.chanceOfRain.T00_06}</p>
            </div>
            <div className="p-2 bg-gray-50 dark:bg-gray-800 rounded">
              <p className="text-xs text-gray-500 dark:text-gray-400">06:00-12:00</p>
              <p className="text-sm font-medium">{forecast.chanceOfRain.T06_12}</p>
            </div>
            <div className="p-2 bg-gray-50 dark:bg-gray-800 rounded">
              <p className="text-xs text-gray-500 dark:text-gray-400">12:00-18:00</p>
              <p className="text-sm font-medium">{forecast.chanceOfRain.T12_18}</p>
            </div>
            <div className="p-2 bg-gray-50 dark:bg-gray-800 rounded">
              <p className="text-xs text-gray-500 dark:text-gray-400">18:00-24:00</p>
              <p className="text-sm font-medium">{forecast.chanceOfRain.T18_24}</p>
            </div>
          </div>
        </div>
      </CardContent>
    </Card>
  )
}
