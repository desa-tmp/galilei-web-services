import ActionBtn from "@/components/action-btn";
import PlanetForm from "@/components/planet-form";
import { deletePlanet, updatePlanet } from "@/lib/actions";
import { api } from "@/lib/api";
import { Page } from "@/lib/types";
import { ApiError } from "api-client";
import { Earth } from "lucide-react";

type PlanetPageProps = Page<{ galaxy_id: string; planet_id: string }>;

export default async function PlanetPage({
  params: { galaxy_id, planet_id },
}: PlanetPageProps) {
  const { data: planet, error: planetError } = await api.GET(
    "/galaxies/{galaxy_id}/planets/{planet_id}",
    {
      params: { path: { galaxy_id, planet_id } },
    }
  );

  if (planetError) {
    throw new ApiError(planetError);
  }

  const { data: stars, error: starsError } = await api.GET(
    "/galaxies/{galaxy_id}/stars",
    {
      params: { path: { galaxy_id } },
    }
  );

  if (starsError) {
    throw new ApiError(starsError);
  }

  return (
    <div className="flex size-full flex-col gap-4">
      <header className="flex items-center gap-4">
        <Earth />
        <h1 className="text-2xl font-bold">{planet.name}</h1>
      </header>
      <PlanetForm
        action={updatePlanet.bind(null, galaxy_id, planet_id)}
        stars={stars}
        planet={{ ...planet, star_id: planet.star_id ?? "" }}
      />
      <ActionBtn
        variant="destructive"
        className="mt-auto"
        action={deletePlanet.bind(null, galaxy_id, planet_id)}
      >
        Delete Star
      </ActionBtn>
    </div>
  );
}
