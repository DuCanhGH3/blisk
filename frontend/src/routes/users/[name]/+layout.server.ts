import { fetchBackend } from "$lib/backend.js";
import type { UserMetadata } from "$lib/types.js";
import { error } from "@sveltejs/kit";

export const load = async (event) => {
  const data = await fetchBackend<UserMetadata>(`/users/${event.params.name}/metadata`, {
    authz: false,
    event,
  });
  if (!data.ok) {
    error(data.status, data.error);
  }
  return {
    data: data.data,
  };
};
