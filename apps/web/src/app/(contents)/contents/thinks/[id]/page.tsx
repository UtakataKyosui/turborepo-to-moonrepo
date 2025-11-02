import { 
    fetchContentById
} from "@workspace/cms/index"
import type { Metadata,ResolvedMetadata } from 'next'

type Props = {
    params: Promise<{ id: string }> 
}

export async function generateMetadata({
    params 
}:Props,parent: ResolvedMetadata): Promise<Metadata> {
    const { id } = await params; 

    const content = await fetchContentById<
        Record<"id" | "createdAt" | "updatedAt" | "title" | "body", string> & {image: Array<string>}
    >(id);

    return {
        title: content.title,
        description: content.body.slice(0, 160),
    }
}



export default async function Page({ params }: Props) {
    const { id } = await params; 
    const content = await fetchContentById<
        Record<"id" | "createdAt" | "updatedAt" | "title" | "body", string> & {image: Array<string>}
    >(id);
    return (
        <div className="prose [&_*]:text-foreground">
            <h3>
                {content.title}
            </h3>
            <div dangerouslySetInnerHTML={{
                __html: content.body
            }} />
        </div>
    )
}