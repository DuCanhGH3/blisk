import { fetchBackend } from "$lib/backend.js";
import type { UserMetadata } from "$lib/types.js";
import { error } from "@sveltejs/kit";

export const load = async ({ cookies, fetch, params, setHeaders }) => {
  const data = await fetchBackend<UserMetadata>(`/users/${params.name}/metadata`, {
    authz: false,
    cookies,
    fetch,
    setHeaders,
  });
  if (!data.ok) {
    error(data.status, data.error);
  }
  return {
    data: data.data,
  };
};
