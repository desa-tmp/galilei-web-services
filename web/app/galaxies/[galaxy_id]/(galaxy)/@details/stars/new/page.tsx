import StarForm from "@/components/star-form";
import { newStar } from "@/lib/actions";
import { Page } from "@/lib/types";

export default async function NewStarPage({
  params: { galaxy_id },
}: Page<{ galaxy_id: string }>) {
  return <StarForm action={newStar.bind(null, galaxy_id)} />;
}
