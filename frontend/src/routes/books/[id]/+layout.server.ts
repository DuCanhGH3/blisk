import { error } from "@sveltejs/kit";
import { fetchBackend } from "$lib/backend";
import type { Book } from "$lib/types";
import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async (event) => {
  const res = await fetchBackend<Book>(`/books/${event.params.id}`, {
    authz: "optional",
    event,
  });
  if (!res.ok) {
    error(res.status, res.error);
  }
  return { book: res.data };
};
