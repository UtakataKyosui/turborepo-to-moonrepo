import { Button } from "@workspace/ui/components/button"
import { Card, CardContent, CardHeader, CardTitle } from "@workspace/ui/components/card"
import { Carousel, CarouselContent, CarouselItem } from "@workspace/ui/components/carousel"

export default function Page() {
  return (
    <main className="container mx-auto py-[10vh]">
      <Card className="border-foreground w-fit flex justify-center mx-auto">
        <CardHeader>
          <CardTitle className="scroll-m-20 text-2xl font-semibold tracking-tight">
            泡沫の研究所へようこそ
          </CardTitle>
        </CardHeader>
        <CardContent>
          私自身が<b>自分自身の管理</b>のために作成したアプリケーションです。
          <br />
          そのため、ここには私に関する情報の多くが含まれています。
        </CardContent>
      </Card>

      <Carousel opts={{
        loop: true
      }}>
        <CarouselContent>
          <CarouselItem>
            <Card className="border-foreground w-fit flex justify-center mx-auto mt-10">
              <CardHeader>
                <CardTitle className="scroll-m-20 text-2xl font-semibold tracking-tight">
                  思想
                </CardTitle>
              </CardHeader>
              <CardContent>
                私の思想や価値観についての考えをここに記しています。<br />
                共感できるかどうかはわかりませんが、<br />
                私が普段喋らない、頭の奥にあるものを垣間見ることができます。
              </CardContent>
            </Card>
          </CarouselItem>
          <CarouselItem>
            <Card className="border-foreground w-fit flex justify-center mx-auto mt-10">
              <CardHeader>
                <CardTitle className="scroll-m-20 text-2xl font-semibold tracking-tight">
                  身体の計測データ
                </CardTitle>
              </CardHeader>
              <CardContent>
                Fitbitから取得したデータを元に、<br />
                可能な限り私の身体の状態を記録しています。<br />
              </CardContent>
            </Card>
          </CarouselItem>
        </CarouselContent>
      </Carousel>
    </main>
  )
}
