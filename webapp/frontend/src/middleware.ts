import { cookies } from "next/headers";
import { NextResponse, NextRequest } from "next/server";
import { validateSessionToken } from "./api/session";

export async function middleware(req: NextRequest) {
  const session = cookies().get("session");

  if (!session) {
    return NextResponse.redirect(new URL("/login", req.url));
  }

  const sessionToken = JSON.parse(session.value).session_token;
  const { is_valid } = await validateSessionToken(sessionToken);

  if (!is_valid) {
    return NextResponse.redirect(new URL("/login", req.url));
  }

  return NextResponse.next();
}

export const config = {
  matcher: ["/", "/orders/:path*"]
};
