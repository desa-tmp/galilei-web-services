"use server";

import { cookies } from "next/headers";
import { Login, LoginSchema, Register, RegisterSchema } from "./schema";
import { parse as parseSetCookies } from "set-cookie-parser";

const SAME_SITE_VALUE = ["lax", "strict", "none"] as const;

type SameSite = (typeof SAME_SITE_VALUE)[number];

function isSameSite(str: string): str is SameSite {
  return SAME_SITE_VALUE.some((v) => v === str);
}

function propagateCookies(res: Response): void {
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
}

export async function login(data: Login) {
  let login_data = LoginSchema.parse(data);

  let res = await fetch("http://127.0.0.1:8080/login", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(login_data),
    credentials: "include",
  });

  propagateCookies(res);

  console.log(await res.json());
}

export async function register(data: Register) {
  let register_data = RegisterSchema.parse(data);

  let res = await fetch("http://127.0.0.1:8080/register", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(register_data),
  });

  propagateCookies(res);

  console.log(await res.json());
}
