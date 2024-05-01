import PlanetForm from "@/components/planet-form";
import { updatePlanet } from "@/lib/actions";
import { fetchApi } from "@/lib/api";
import { Planet } from "@/lib/schema";
import { Page } from "@/lib/types";

type PlanetPageProps = Page<{ galaxy_id: string; planet_id: string }>;

export default async function PlanetPage({
  params: { galaxy_id, planet_id },
}: PlanetPageProps) {
  const planet = (await (
    await fetchApi(`/galaxies/${galaxy_id}/planets/${planet_id}`)
  ).json()) as Planet;

  return (
    <PlanetForm
      action={updatePlanet.bind(null, galaxy_id, planet_id)}
      planet={planet}
    />
  );
}
