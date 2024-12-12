import { fetchBackend } from "$lib/backend";
import type { ReadingTracker } from "$lib/types.js";
import { error } from "@sveltejs/kit";

export const load = async (event) => {
  const res = await fetchBackend<ReadingTracker>(`/books/tracker/${event.params.id}`, {
    authz: true,
    event,
  });

  if (!res.ok) {
    error(res.status, res.error);
  }

  return { data: res.data };
};
