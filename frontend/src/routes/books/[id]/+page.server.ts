import { fetchBackend } from "$lib/backend";
import { error } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";
import type { BookVolumeWithEntries } from "$lib/types";

export const load: PageServerLoad = async (event) => {
  const res = await fetchBackend<BookVolumeWithEntries[]>(`/books/${event.params.id}/volumes`, {
    authz: false,
    event,
  });
  if (!res.ok) {
    error(res.status, res.error);
  }
  return { volumes: res.data };
};
