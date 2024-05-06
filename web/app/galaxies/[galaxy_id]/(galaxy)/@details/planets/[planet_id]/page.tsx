import PlanetForm from "@/components/planet-form";
import { updatePlanet } from "@/lib/actions";
import { fetchApi } from "@/lib/api";
import { Planet, Star } from "@/lib/schema";
import { Page } from "@/lib/types";
import { Earth } from "lucide-react";

type PlanetPageProps = Page<{ galaxy_id: string; planet_id: string }>;

export default async function PlanetPage({
  params: { galaxy_id, planet_id },
}: PlanetPageProps) {
  const planet = (await (
    await fetchApi(`/galaxies/${galaxy_id}/planets/${planet_id}`)
  ).json()) as Planet;

  const stars = (await (
    await fetchApi(`/galaxies/${galaxy_id}/stars`)
  ).json()) as Star[];

  return (
    <div className="size-full">
      <header className="flex items-center gap-4 pb-4">
        <Earth />
        <h1 className="text-2xl font-bold">{planet.name}</h1>
      </header>
      <PlanetForm
        action={updatePlanet.bind(null, galaxy_id, planet_id)}
        stars={stars}
        planet={{ ...planet, star_id: planet.star_id ?? "" }}
      />
    </div>
  );
}
