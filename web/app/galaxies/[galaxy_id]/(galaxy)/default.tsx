import { Page } from "@/lib/types";

type GalaxyPage = Page<{ galaxy_id: string }>;

export default function Galaxy({ params: { galaxy_id } }: GalaxyPage) {
  return <h1>Galaxy id: {galaxy_id}</h1>;
}
