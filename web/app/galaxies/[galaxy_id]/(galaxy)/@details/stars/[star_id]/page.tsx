import StarForm from "@/components/star-form";
import { updateStar } from "@/lib/actions";
import { fetchApi } from "@/lib/api";
import { Star } from "@/lib/schema";
import { Page } from "@/lib/types";

type StarPageProps = Page<{ galaxy_id: string; star_id: string }>;

export default async function StarPage({
  params: { galaxy_id, star_id },
}: StarPageProps) {
  const star = (await (
    await fetchApi(`/galaxies/${galaxy_id}/stars/${star_id}`)
  ).json()) as Star;

  return (
    <StarForm action={updateStar.bind(null, galaxy_id, star_id)} star={star} />
  );
}
