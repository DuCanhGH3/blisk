import { fetchBackend } from "$lib/backend";
import type { User } from "$lib/types";
import type { Handle } from "@sveltejs/kit";

export const handle: Handle = async ({ event, resolve }) => {
  const token = event.cookies.get("token");
  // Since we only use `event.locals.user` on the client (we authenticate the end-user to the server
  // via the JWT token in other cases), we limit this loading to GET requests only.
  if (event.request.method === "GET" && token) {
    const res = await fetchBackend<User>("/auth/authenticate", {
      authz: true,
      event,
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
