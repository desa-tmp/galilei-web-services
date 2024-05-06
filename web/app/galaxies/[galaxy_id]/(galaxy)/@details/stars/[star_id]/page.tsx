import StarForm from "@/components/star-form";
import { updateStar } from "@/lib/actions";
import { fetchApi } from "@/lib/api";
import { Star } from "@/lib/schema";
import { Page } from "@/lib/types";
import { Star as StarIcon } from "lucide-react";

type StarPageProps = Page<{ galaxy_id: string; star_id: string }>;

export default async function StarPage({
  params: { galaxy_id, star_id },
}: StarPageProps) {
  const star = (await (
    await fetchApi(`/galaxies/${galaxy_id}/stars/${star_id}`)
  ).json()) as Star;

  return (
    <div className="size-full">
      <header className="flex items-center gap-4 pb-4">
        <StarIcon />
        <h1 className="text-2xl font-bold">{star.name}</h1>
      </header>
      <StarForm
        action={updateStar.bind(null, galaxy_id, star_id)}
        star={star}
      />
    </div>
  );
}
