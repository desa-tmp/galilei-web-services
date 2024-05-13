import ActionBtn from "@/components/action-btn";
import GalaxyForm from "@/components/galaxy-form";
import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";
import { deleteGalaxy, updateGalaxy } from "@/lib/actions";
import { api } from "@/lib/api";
import { Page } from "@/lib/types";
import { ApiError } from "api-client";

type GalaxySettingsPageProps = Page<{ galaxy_id: string }>;

export default async function GalaxySettingsPage({
  params: { galaxy_id },
}: GalaxySettingsPageProps) {
  const { data, error } = await api.GET("/galaxies/{galaxy_id}", {
    params: { path: { galaxy_id } },
  });

  if (error) {
    throw new ApiError(error);
  }

  return (
    <div className="flex flex-col gap-6">
      <GalaxyForm
        action={updateGalaxy.bind(null, galaxy_id)}
        galaxy={data.galaxy}
      />
      <Separator size={2}>
        <span className="bg-background p-1">OR</span>
      </Separator>
      <Button variant="destructive" className="w-full" asChild>
        <ActionBtn action={deleteGalaxy.bind(null, galaxy_id)}>
          Delete Galaxy
        </ActionBtn>
      </Button>
    </div>
  );
}
