import PlanetForm from "@/components/planet-form";
import { newPlanet } from "@/lib/actions";
import { fetchApi } from "@/lib/api";
import { Star } from "@/lib/schema";
import { Page } from "@/lib/types";

export default async function NewPlanetPage({
  params: { galaxy_id },
}: Page<{ galaxy_id: string }>) {
  const stars = (await (
    await fetchApi(`/galaxies/${galaxy_id}/stars`)
  ).json()) as Star[];

  return <PlanetForm action={newPlanet.bind(null, galaxy_id)} stars={stars} />;
}
