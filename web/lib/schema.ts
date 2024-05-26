import { ZodType, z } from "zod";
import { components } from "api-client";

export type Login = components["schemas"]["AuthData"];

export const LoginSchema = z.object({
  username: z.string().min(1, { message: "Username is required" }),
  password: z.string().min(1, { message: "Password is required" }),
  remember: z.boolean(),
}) satisfies ZodType<Login>;

export type Register = components["schemas"]["AuthData"] & { confirm: string };

export const RegisterSchema = z
  .object({
    username: z.string().min(1, { message: "Username is required" }),
    password: z.string().min(1, { message: "Password is required" }),
    // confirm password
    confirm: z.string().min(1, { message: "Confirm password is required" }),
    remember: z.boolean(),
  })
  .refine((data) => data.password === data.confirm, {
    message: "Passwords don't match",
    path: ["confirm"],
  }) satisfies ZodType<Register>;

export type User = components["schemas"]["User"];

export type Galaxy = components["schemas"]["Galaxy"];

export type GalaxyData = components["schemas"]["CreateGalaxyData"];

export const GalaxyDataSchema = z.object({
  name: z.string().min(1, {
    message: "Galaxy name is required",
  }),
}) satisfies ZodType<GalaxyData>;

export type Star = components["schemas"]["Star"];
export type StarStatus = components["schemas"]["StarStatus"];

export type StarData = Omit<
  components["schemas"]["CreateStarData"],
  "public_domain"
> & {
  public_domain: string;
};

export const StarDataSchema = z.object({
  name: z.string().min(1, {
    message: "Star name is required",
  }),
  nebula: z.string().min(1, {
    message: "Star nebula is required",
  }),
  public_domain: z.string(),
}) satisfies ZodType<StarData>;

export type UpdateStarData = Omit<
  components["schemas"]["UpdateStarData"],
  "public_domain"
> & {
  public_domain?: string;
};

export const UpdateStarDataSchema =
  StarDataSchema.partial() satisfies ZodType<UpdateStarData>;

export type Planet = components["schemas"]["Planet"];

export type PlanetData = Omit<
  components["schemas"]["CreatePlanetData"],
  "star"
> & {
  star_id: string;
};

export const PlanetDataSchema = z.object({
  name: z.string().min(1, {
    message: "Planet name is required",
  }),
  capacity: z.coerce.number().min(0, {
    message: "Minimum planet capacity is 0",
  }),
  path: z.coerce.string().min(0, {
    message: "Minimum planet capacity is 0",
  }),
  star_id: z.string(),
}) satisfies ZodType<PlanetData>;

export type UpdatePlanetData = Omit<
  components["schemas"]["UpdatePlanetData"],
  "star"
> & {
  star_id?: string;
};

export const UpdatePlanetDataSchema =
  PlanetDataSchema.partial() satisfies ZodType<UpdatePlanetData>;
