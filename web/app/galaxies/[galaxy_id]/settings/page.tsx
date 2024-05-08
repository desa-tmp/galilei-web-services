import GalaxyForm from "@/components/galaxy-form";
import { updateGalaxy } from "@/lib/actions";
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
    <GalaxyForm
      action={updateGalaxy.bind(null, galaxy_id)}
      galaxy={data.galaxy}
    />
  );
}
