import {
    fetchContents
} from "@workspace/cms/index"
import { Button } from "@workspace/ui/components/button";
import {
  Item,
  ItemActions,
  ItemContent,
  ItemDescription,
  ItemMedia,
  ItemTitle,
} from "@workspace/ui/components/item"
import Link from "next/link";
import type { Metadata } from "next/types";

export const metadata: Metadata = {
    title: "Thinks",
    description: "A collection of thoughts and ideas.",
}

export default async function Thinks() {

    const contents = await fetchContents<{
        id: string;
        title: string;
    }>();
    console.log(contents);

    return (
        <>
            {contents.contents.map((content) => (
                <Item key={content.id} className="my-4 border-foreground/60 justify-between">
                    <ItemTitle>
                        {content.title}
                    </ItemTitle>
                    <ItemActions>
                        <Button asChild className="font-bold">
                            <Link href={`/contents/thinks/${content.id}`}>
                                詳細を見る
                            </Link>
                        </Button>
                    </ItemActions>
                </Item>
            ))}
        </>
    )
}