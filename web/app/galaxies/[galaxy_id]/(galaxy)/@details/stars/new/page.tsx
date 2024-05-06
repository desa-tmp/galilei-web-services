import StarForm from "@/components/star-form";
import { newStar } from "@/lib/actions";
import { Page } from "@/lib/types";
import { Star } from "lucide-react";

export default async function NewStarPage({
  params: { galaxy_id },
}: Page<{ galaxy_id: string }>) {
  return (
    <div className="size-full">
      <header className="flex items-center gap-4 pb-4">
        <Star />
        <h1 className="text-2xl font-bold">New Star</h1>
      </header>
      <StarForm action={newStar.bind(null, galaxy_id)} />
    </div>
  );
}
