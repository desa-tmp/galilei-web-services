import { NextResponse } from "next/server";
import type { NextRequest } from "next/server";
import { is_authorized } from "./lib/auth";

const AUTH_PATHS = ["/login", "/register"] as const;

export async function middleware(request: NextRequest) {
  const is_auth = await is_authorized();

  const is_auth_path = AUTH_PATHS.some((path) =>
    request.nextUrl.pathname.startsWith(path)
  );

  const url = request.nextUrl.clone();

  if (is_auth && is_auth_path) {
    url.pathname = "/galaxies";
    return NextResponse.redirect(url);
  }

  if (!is_auth && !is_auth_path) {
    url.pathname = "/login";
    return NextResponse.redirect(url);
  }
}

export const config = {
  matcher: [
    /*
     * Match all request paths except for the ones starting with:
     * - _next/static (static files)
     * - _next/image (image optimization files)
     * - favicon.ico (favicon file)
     */
    "/((?!_next/static|_next/image|favicon.ico).*)",
  ],
};
