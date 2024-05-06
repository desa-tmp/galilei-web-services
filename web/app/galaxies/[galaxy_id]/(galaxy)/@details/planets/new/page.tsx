import PlanetForm from "@/components/planet-form";
import { newPlanet } from "@/lib/actions";
import { fetchApi } from "@/lib/api";
import { Star } from "@/lib/schema";
import { Page } from "@/lib/types";
import { Earth } from "lucide-react";

export default async function NewPlanetPage({
  params: { galaxy_id },
}: Page<{ galaxy_id: string }>) {
  const stars = (await (
    await fetchApi(`/galaxies/${galaxy_id}/stars`)
  ).json()) as Star[];

  return (
    <div className="size-full">
      <header className="flex items-center gap-4 pb-4">
        <Earth />
        <h1 className="text-2xl font-bold">New Planet</h1>
      </header>
      <PlanetForm action={newPlanet.bind(null, galaxy_id)} stars={stars} />
    </div>
  );
}
