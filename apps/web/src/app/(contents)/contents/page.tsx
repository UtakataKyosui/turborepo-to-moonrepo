import { Button } from "@workspace/ui/components/button";
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "@workspace/ui/components/card";
import Link from "next/dist/client/link";

export default function ContentsDetailsPage() {
    return (
        <section className="container mx-auto py-5">
            <h3 className="text-foreground text-2xl font-bold underline underline-offset-8">
                開発コンテンツ一覧ページ
            </h3>
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 my-5 gap-5">
                {
                    [1, 2, 3, 4, 5, 6].map((item) => (
                        <Card key={item} className="border-foreground m-2">
                            <CardHeader>
                                <CardTitle>
                                    コンテンツ {item}
                                </CardTitle>
                                <CardDescription>
                                    これはコンテンツ {item} の説明です。
                                </CardDescription>
                            </CardHeader>
                            <CardContent>
                                ここにコンテンツ {item} の詳細情報が表示されます。
                            </CardContent>
                            <CardFooter>
                                <Button asChild className="font-bold">
                                    <Link href={`/contents/${item}`}>
                                        詳細を見る
                                    </Link>
                                </Button>
                            </CardFooter>
                        </Card>
                    ))
                }
            </div>
        </section>
    )
}