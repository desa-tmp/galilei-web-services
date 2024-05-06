import { ZodType, z } from "zod";

export const LoginSchema = z.object({
  username: z.string().min(1, { message: "Username is required" }),
  password: z.string().min(1, { message: "Password is required" }),
  remember: z.boolean(),
});

export type Login = z.infer<typeof LoginSchema>;

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
  });

export type Register = z.infer<typeof RegisterSchema>;

export interface User {
  id: string;
  name: string;
}

export interface Galaxy {
  id: string;
  name: string;
  user_id: string;
}

export type NewGalaxy = Pick<Galaxy, "name">;

export const NewGalaxySchema = z.object({
  name: z.string().min(1, {
    message: "Galaxy name is required",
  }),
}) satisfies ZodType<NewGalaxy>;

export interface Star {
  id: string;
  name: string;
  nebula: string;
  galaxy_id: string;
}

export type StarData = Pick<Star, "name" | "nebula">;

export const StarDataSchema = z.object({
  name: z.string().min(1, {
    message: "Star name is required",
  }),
  nebula: z.string().min(1, {
    message: "Star nebula is required",
  }),
}) satisfies ZodType<StarData>;

export interface Planet {
  id: string;
  name: string;
  capacity: number;
  star_id: string | null;
  galaxy_id: string;
}

export type PlanetData = Pick<Planet, "name" | "capacity"> & {
  star_id: string;
};

export const PlanetDataSchema = z.object({
  name: z.string().min(1, {
    message: "Planet name is required",
  }),
  capacity: z.coerce.number().min(0, {
    message: "Minimum planet capacity is 0",
  }),
  star_id: z.string(),
}) satisfies ZodType<PlanetData>;
