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
