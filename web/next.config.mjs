/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  transpilePackages: ["api-client"],
  async rewrites() {
    return [{ source: "/", destination: "/galaxies" }];
  },
};

export default nextConfig;
