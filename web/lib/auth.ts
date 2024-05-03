import { fetchApi } from "@/lib/api";

const UNAUTHORIZED_STATUS_CODE = 401;

export async function is_authorized(): Promise<boolean> {
  const res = await fetchApi("/auth/verify");

  return res.status !== UNAUTHORIZED_STATUS_CODE;
}
