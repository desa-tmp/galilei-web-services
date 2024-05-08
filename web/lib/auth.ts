import { api } from "@/lib/api";

export async function is_authorized(): Promise<boolean> {
  const { error } = await api.GET("/auth/verify");

  return error == undefined;
}
