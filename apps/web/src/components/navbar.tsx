import { ModeToggle } from "./theme-toggle";

export default function Navbar() {
    return (
        <header className="fixed flex justify-between items-center w-full p-5">
            <div />
            <h1 className="scroll-m-20 text-center text-4xl font-extrabold tracking-tight text-balance">
                泡沫の研究所
            </h1>
            <ModeToggle />
        </header>
    )
}