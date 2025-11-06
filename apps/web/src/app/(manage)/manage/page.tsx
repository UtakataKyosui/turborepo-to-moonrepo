// import getWeather from "@workspace/weather/index"
import { LaundryAdvisor } from "@/components/laundry-advisor"

export default async function ManageAppLayout() {
    // const weather = await getWeather();

    return (
        <section className="space-y-6">
            <h3 className="text-2xl font-bold mb-4">
                Manage Page
            </h3>

            {/* 洗濯物診断セクション */}
            <div className="space-y-4">
                <h4 className="text-xl font-semibold">洗濯物診断</h4>
                {/* <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
                    {weather.forecasts.map((forecast) => (
                        <LaundryAdvisor key={forecast.date} forecast={forecast} />
                    ))}
                </div> */}
            </div>
        </section>
    )
}
