import type { NextConfig } from "next"

const nextConfig: NextConfig = {
  transpilePackages: [
    "@workspace/ui",
    "@workspace/cms",
    "@workspace/fitbit",
  ],
}

export default nextConfig
