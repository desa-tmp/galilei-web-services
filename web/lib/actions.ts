"use server";

import {
  Login,
  LoginSchema,
  NewGalaxy,
  NewGalaxySchema,
  Register,
  RegisterSchema,
} from "./schema";
import { fetchApi } from "./api";

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
