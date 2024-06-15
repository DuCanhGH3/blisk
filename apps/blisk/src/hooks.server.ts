import { fetchBackend } from "$lib/backend";
import type { User } from "$lib/types";
import type { Handle } from "@sveltejs/kit";

export const handle: Handle = async ({ event, resolve }) => {
  const tokenType = event.cookies.get("token_type");
  const token = event.cookies.get("token");
  // Since we only use `event.locals.user` on the client (we authenticate the end-user to the server
  // via the JWT token in other cases), we limit this loading to GET requests only.
  if (event.request.method === "GET" && tokenType && token) {
    const res = await fetchBackend<User>("/users/authenticate", {
      authz: true,
      cookies: event.cookies,
      fetch: event.fetch,
      setHeaders: event.setHeaders,
      method: "POST",
      signal: AbortSignal.timeout(5000),
    });
    if (res.ok) {
      event.locals.user = res.data;
    } else {
      console.error("An error occurred while fetching user:", res.status, res.error);
    }
  }
  return resolve(event);
};
