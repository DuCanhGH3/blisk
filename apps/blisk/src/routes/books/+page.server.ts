import { fetchBackend } from "$lib/backend";
import { error } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";
import type { Book } from "$lib/types";

export const load: PageServerLoad = async ({ cookies, fetch, setHeaders }) => {
  const res = await fetchBackend<Book[]>("/books", {
    authz: "optional",
    cookies,
    fetch,
    setHeaders,
  });
  if (!res.ok) {
    error(res.status, res.error);
  }
  return { books: res.data };
};
