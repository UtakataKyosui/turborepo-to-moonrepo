"use client"
import { Avatar, AvatarImage, AvatarFallback } from "@workspace/ui/components/avatar";
import { Button } from "@workspace/ui/components/button";
import Link from "next/link";
import { useTheme } from "next-themes";

export default function Footer() {
    const { theme } = useTheme();

    return (
        <footer className="border-t-1 border-foreground top-[90vh] w-full py-5">
          <div className="flex gap-4">
            <Button asChild> 
                <Link className="border w-fit p-2 rounded-lg bg-foreground text-background font-bold" href="https://twitter.com/utakatakyosui">
                    <Avatar className="rounded-none size-5">
                        <AvatarImage src={`https://cdn.cms-twdigitalassets.com/content/dam/about-twitter/x/brand-toolkit/logo-${theme === "dark" ? "black" : "white"}.png.twimg.2560.png`} />
                    </Avatar>
                    @utakatakyosui
                </Link>
            </Button>
            <Button asChild>
                <Link className="bg-foreground w-fit rounded-lg p-2 font-bold text-green-300" href="https://misskey.systems/@utakata">
                    <Avatar className="rounded-none size-5">
                        <AvatarImage src="https://storage.misskey.systems/storage/files/489b622c-bf02-49a6-a7cc-c67ba1f0930f.png" />
                    </Avatar>
                    @utakata
                </Link>
            </Button>
          </div>
          <div className="flex items-center justify-center h-full">
            <Avatar>
                <AvatarImage src="https://github.com/utakatakyosui.png" />
                <AvatarFallback>UK</AvatarFallback>
            </Avatar>
            <span className="ml-2 underline text-foreground text-xl">©{new Date().getFullYear()} Utakata Kyosui</span>
          </div>
        </footer>
    )
}