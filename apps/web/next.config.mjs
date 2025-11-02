/** @type {import('next').NextConfig} */
const nextConfig = {
  transpilePackages: [
    "@workspace/ui",
    "@workspace/cms",
    "@workspace/fitbit",
  ],
}

export default nextConfig
