"use server";

import {
  Galaxy,
  Login,
  LoginSchema,
  NewGalaxy,
  NewGalaxySchema,
  Planet,
  PlanetData,
  PlanetDataSchema,
  Register,
  RegisterSchema,
  Star,
  StarData,
  StarDataSchema,
} from "./schema";
import { fetchApi } from "./api";
import { revalidateTag } from "next/cache";
import { redirect } from "next/navigation";

export async function login(data: Login) {
  const login_data = LoginSchema.parse(data);

  await fetchApi("/auth/login", {
    method: "POST",
    body: login_data,
  });

  redirect("/galaxies");
}

export async function register(data: Register) {
  const register_data = RegisterSchema.parse(data);

  await fetchApi("/auth/register", {
    method: "POST",
    body: register_data,
  });

  redirect("/galaxies");
}

export async function newGalaxy(data: NewGalaxy) {
  const new_galaxy = NewGalaxySchema.parse(data);

  const res = await fetchApi("/galaxies", {
    method: "POST",
    body: new_galaxy,
  });

  const galaxy: Galaxy = await res.json();

  redirect(`/galaxies/${galaxy.id}`);
}

export async function newStar(galaxyId: string, data: StarData) {
  const star_data = StarDataSchema.parse(data);

  const res = await fetchApi(`/galaxies/${galaxyId}/stars`, {
    method: "POST",
    body: star_data,
  });

  const star: Star = await res.json();

  revalidateTag("galaxy");
  redirect(`/galaxies/${star.galaxy_id}/stars/${star.id}`);
}

export async function updateStar(
  galaxyId: string,
  starId: string,
  data: StarData
) {
  const star_data = StarDataSchema.parse(data);

  await fetchApi(`/galaxies/${galaxyId}/stars/${starId}`, {
    method: "PUT",
    body: star_data,
  });

  revalidateTag("galaxy");
}

export async function newPlanet(galaxyId: string, data: PlanetData) {
  const { star_id, ...rest } = PlanetDataSchema.parse(data);

  const res = await fetchApi(`/galaxies/${galaxyId}/planets`, {
    method: "POST",
    body: { ...rest, star: { id: star_id.length === 0 ? null : star_id } },
  });

  const planet: Planet = await res.json();

  revalidateTag("galaxy");
  redirect(`/galaxies/${planet.galaxy_id}/planets/${planet.id}`);
}

export async function updatePlanet(
  galaxyId: string,
  planetId: string,
  data: PlanetData
) {
  const { star_id, ...rest } = PlanetDataSchema.parse(data);

  await fetchApi(`/galaxies/${galaxyId}/planets/${planetId}`, {
    method: "PUT",
    body: { ...rest, star: { id: star_id.length === 0 ? null : star_id } },
  });

  revalidateTag("galaxy");
}
