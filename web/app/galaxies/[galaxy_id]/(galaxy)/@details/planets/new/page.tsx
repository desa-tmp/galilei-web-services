import PlanetForm from "@/components/planet-form";
import { newPlanet } from "@/lib/actions";
import { api } from "@/lib/api";
import { Page } from "@/lib/types";
import { ApiError } from "api-client";
import { Earth } from "lucide-react";

export default async function NewPlanetPage({
  params: { galaxy_id },
}: Page<{ galaxy_id: string }>) {
  const { data: stars, error } = await api.GET("/galaxies/{galaxy_id}/stars", {
    params: { path: { galaxy_id } },
  });

  if (error) {
    throw new ApiError(error);
  }

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
