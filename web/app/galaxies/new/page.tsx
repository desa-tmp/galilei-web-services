import GalaxyForm from "@/components/galaxy-form";
import { newGalaxy } from "@/lib/actions";

export default async function NewGalaxyPage() {
  return <GalaxyForm action={newGalaxy} />;
}
