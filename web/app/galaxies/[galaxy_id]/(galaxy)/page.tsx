import Galaxy from "@/components/galaxy";
import { Page } from "@/lib/types";

type GalaxyPageProps = Page<{ galaxy_id: string }>;

export default async function GalaxyPage({
  params: { galaxy_id },
}: GalaxyPageProps) {
  return <Galaxy galaxy_id={galaxy_id} />;
}
