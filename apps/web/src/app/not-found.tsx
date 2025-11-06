import { Button } from "@workspace/ui/components/button";
import Link from "next/link";

export default function NotFound() {
    return (
        <main className="flex min-h-screen flex-col items-center justify-center gap-5">
            <h1 className="text-foreground scroll-m-20 text-center text-4xl font-extrabold tracking-tight text-balance">
                Not Found
            </h1>
            <p>
                Could not find requested resource. Go back to{' '}
            </p>
            <Button asChild>
                <Link href="/">
                    Home Page
                </Link>
            </Button>
        </main>
    )
}