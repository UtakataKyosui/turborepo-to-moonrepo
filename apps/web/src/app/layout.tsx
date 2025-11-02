import { Inter } from "next/font/google"

import "@workspace/ui/globals.css"
import { Providers } from "@/components/providers"
import Navbar from "../components/navbar"
import Footer from "../components/footer"
// import AutoBread from "../components/auto-bread"
import { SidebarProvider } from "@workspace/ui/components/sidebar"
// import AppSlidebar from "../components/app-slidebar"

const fontSans = Inter({
  subsets: ["latin"],
  variable: "--font-sans",
})

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode
}>) {
  return (
    <html lang="en" suppressHydrationWarning>
      <body
        className={`${fontSans.variable} font-sans antialiased `}
      >
        <Providers>
          <SidebarProvider>
            {/* <AppSlidebar /> */}
            <main className="container mx-auto py-[10vh]">
              <Navbar />
              {/* <AutoBread /> */}
              {children}
              <Footer />
            </main>
          </SidebarProvider>
        </Providers>
      </body>
    </html>
  )
}
