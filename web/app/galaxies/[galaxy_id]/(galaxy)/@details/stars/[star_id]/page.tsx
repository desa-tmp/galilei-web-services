import ActionBtn from "@/components/action-btn";
import StarForm from "@/components/star-form";
import { deleteStar, updateStar } from "@/lib/actions";
import { api } from "@/lib/api";
import { Page } from "@/lib/types";
import { ApiError } from "api-client";
import { Star as StarIcon } from "lucide-react";

type StarPageProps = Page<{ galaxy_id: string; star_id: string }>;

export default async function StarPage({
  params: { galaxy_id, star_id },
}: StarPageProps) {
  const { data: star, error } = await api.GET(
    "/galaxies/{galaxy_id}/stars/{star_id}",
    {
      params: { path: { galaxy_id, star_id } },
    }
  );

  if (error) {
    throw new ApiError(error);
  }

  return (
    <div className="flex size-full flex-col gap-4">
      <header className="flex items-center gap-4">
        <StarIcon />
        <h1 className="text-2xl font-bold">{star.name}</h1>
      </header>
      <StarForm
        action={updateStar.bind(null, galaxy_id, star_id)}
        star={star}
      />
      <ActionBtn
        variant="destructive"
        className="mt-auto"
        action={deleteStar.bind(null, galaxy_id, star_id)}
      >
        Delete Star
      </ActionBtn>
    </div>
  );
}
