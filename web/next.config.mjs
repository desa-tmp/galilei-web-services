/** @type {import('next').NextConfig} */
const nextConfig = {
  async rewrites() {
    return [{ source: "/", destination: "/galaxies" }];
  },
};

export default nextConfig;
