import { Page } from "@/lib/types";

type PlanetPage = Page<{ galaxy_id: string; planet_id: string }>;

export default function Planet({
  params: { galaxy_id, planet_id },
}: PlanetPage) {
  return (
    <h1>
      Planet id: {planet_id} in galaxy id: {galaxy_id}
    </h1>
  );
}
