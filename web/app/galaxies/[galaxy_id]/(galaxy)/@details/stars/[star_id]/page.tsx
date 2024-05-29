import StarStatus from "@/components/StarStatus";
import ActionBtn from "@/components/action-btn";
import GenericStarForm from "@/components/generic-star-form";
import NetworkStarForm from "@/components/network-star-form";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { deleteStar } from "@/lib/actions";
import { api } from "@/lib/api";
import { Star } from "@/lib/schema";
import { Page } from "@/lib/types";
import { ApiError } from "api-client";
import { Star as StarIcon, Trash2 } from "lucide-react";

type StarPageProps = Page<{ galaxy_id: string; star_id: string }>;

export default async function StarPage({
  params: { galaxy_id, star_id },
}: StarPageProps) {
  const { data, error } = await api.GET(
    "/galaxies/{galaxy_id}/stars/{star_id}",
    {
      params: { path: { galaxy_id, star_id } },
      headers: new Headers({ "Content-Type": "application/json" }),
    }
  );

  // TODO improve type checking at runtime
  // the star type is returned by 'Content-Type': 'application/json' header
  const star = data as Star;

  if (error) {
    throw new ApiError(error);
  }

  return (
    <div className="flex size-full flex-col gap-4">
      <header className="flex items-center justify-between">
        <div className="flex items-center gap-4">
          <StarIcon />
          <h1 className="text-2xl font-bold">{star.name}</h1>
        </div>
        <ActionBtn
          variant="destructive"
          size="icon"
          action={deleteStar.bind(null, galaxy_id, star_id)}
        >
          <Trash2 />
        </ActionBtn>
      </header>
      <StarStatus galaxy_id={galaxy_id} star_id={star_id} withLabel />
      <Tabs defaultValue="generic">
        <TabsList className="w-full">
          <TabsTrigger value="generic" className="flex-1">
            Generic
          </TabsTrigger>
          <TabsTrigger value="network" className="flex-1">
            Network
          </TabsTrigger>
        </TabsList>
        <TabsContent value="generic">
          <GenericStarForm
            galaxyId={galaxy_id}
            starId={star_id}
            genericData={star}
          />
        </TabsContent>
        <TabsContent value="network">
          <NetworkStarForm
            galaxyId={galaxy_id}
            starId={star_id}
            networkData={{ ...star, public_domain: star.public_domain ?? "" }}
          />
        </TabsContent>
      </Tabs>
    </div>
  );
}
