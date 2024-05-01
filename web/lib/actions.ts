"use server";

import {
  Login,
  LoginSchema,
  NewGalaxy,
  NewGalaxySchema,
  PlanetData,
  PlanetDataSchema,
  Register,
  RegisterSchema,
  StarData,
  StarDataSchema,
} from "./schema";
import { fetchApi } from "./api";
import { revalidateTag } from "next/cache";

export async function login(data: Login) {
  const login_data = LoginSchema.parse(data);

  const res = await fetchApi("/login", {
    method: "POST",
    body: login_data,
  });

  console.log(await res.json());
}

export async function register(data: Register) {
  const register_data = RegisterSchema.parse(data);

  const res = await fetchApi("/register", {
    method: "POST",
    body: register_data,
  });

  console.log(await res.json());
}

export async function newGalaxy(data: NewGalaxy) {
  const new_galaxy = NewGalaxySchema.parse(data);

  const res = await fetchApi("/galaxies", {
    method: "POST",
    body: new_galaxy,
  });

  console.log(await res.json());
}

export async function newStar(galaxyId: string, data: StarData) {
  const star_data = StarDataSchema.parse(data);

  const res = await fetchApi(`/galaxies/${galaxyId}/stars`, {
    method: "POST",
    body: star_data,
  });

  console.log(await res.json());
  revalidateTag("galaxy");
}

export async function updateStar(
  galaxyId: string,
  starId: string,
  data: StarData
) {
  const star_data = StarDataSchema.parse(data);

  const res = await fetchApi(`/galaxies/${galaxyId}/stars/${starId}`, {
    method: "PUT",
    body: star_data,
  });

  console.log(await res.json());
  revalidateTag("galaxy");
}

export async function newPlanet(galaxyId: string, data: PlanetData) {
  const { star_id, ...rest } = PlanetDataSchema.parse(data);

  const res = await fetchApi(`/galaxies/${galaxyId}/planets`, {
    method: "POST",
    body: { ...rest, star: { id: star_id.length === 0 ? null : star_id } },
  });

  console.log(await res.json());
  revalidateTag("galaxy");
}

export async function updatePlanet(
  galaxyId: string,
  planetId: string,
  data: PlanetData
) {
  const { star_id, ...rest } = PlanetDataSchema.parse(data);

  const res = await fetchApi(`/galaxies/${galaxyId}/planets/${planetId}`, {
    method: "PUT",
    body: { ...rest, star: { id: star_id.length === 0 ? null : star_id } },
  });

  console.log(await res.json());
  revalidateTag("galaxy");
}
