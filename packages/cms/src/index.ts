import {
    createClient
} from "microcms-js-sdk"

const serviceDomain = process.env.MICROCMS_SERVICE_DOMAIN
const apiKey = process.env.MICROCMS_API_KEY
const endpoint = process.env.MICROCMS_ENDPOINT

if(
    typeof serviceDomain !== "string" ||
    typeof apiKey !== "string" ||
    typeof endpoint !== "string"
) {
    throw new Error("Missing MICROCMS environment variables")
}

export const client = createClient({
    serviceDomain,
    apiKey,
})

export const fetchContents = async <T>() => {
    const data = await client.getList<T>({
        endpoint,
        queries: {
            fields: "id,title",
            limit: 10
        }
    })
    return data
}

export const fetchContentById = async <T>(id: string,draftKey?: string) => {
    const data = await client.get<T>({
        endpoint,
        contentId: id,
        queries: {
            draftKey
        }
    })
    return data
}