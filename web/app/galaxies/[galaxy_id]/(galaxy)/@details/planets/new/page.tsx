import PlanetForm from "@/components/planet-form";
import { newPlanet } from "@/lib/actions";
import { Page } from "@/lib/types";

export default async function NewPlanetPage({
  params: { galaxy_id },
}: Page<{ galaxy_id: string }>) {
  return <PlanetForm action={newPlanet.bind(null, galaxy_id)} />;
}
