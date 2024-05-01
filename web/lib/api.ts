"use server";

import { cookies } from "next/headers";
import { parse as parseSetCookies } from "set-cookie-parser";

const SAME_SITE_VALUE = ["lax", "strict", "none"] as const;

type SameSite = (typeof SAME_SITE_VALUE)[number];

function isSameSite(str: string): str is SameSite {
  return SAME_SITE_VALUE.some((v) => v === str);
}

interface FetchOptions {
  method?: string;
  body?: unknown;
  tags?: string[];
}

export async function fetchApi(
  path: string,
  { method = "GET", body = null, tags }: FetchOptions = {}
): Promise<Response> {
  const res = await fetch(
    `http://127.0.0.1:8080/${path.startsWith("/") ? path.slice(1) : path}`,
    {
      method,
      headers: {
        "Content-Type": "application/json",
        Cookie: cookies().toString(),
      },
      body: body ? JSON.stringify(body) : null,
      next: {
        tags,
      },
    }
  );

  let resCookies = parseSetCookies(res.headers.getSetCookie());

  for (const { name, value, sameSite, ...options } of resCookies) {
    if (sameSite && isSameSite(sameSite)) {
      cookies().set(name, value, { sameSite, ...options });
    } else if (!sameSite) {
      cookies().set(name, value, options);
    } else {
      throw new Error("Invalid Same Site value");
    }
  }

  return res;
}
