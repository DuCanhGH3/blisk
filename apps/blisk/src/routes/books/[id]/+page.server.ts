import { fetchBackend } from "$lib/backend";
import { error } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";
import type { Book } from "$lib/types";

export const load: PageServerLoad = async ({ cookies, params, setHeaders }) => {
  const res = await fetchBackend<Book>(`/books/read?book_id=${params.id}`, {
    authz: false,
    cookies,
    fetch,
    setHeaders,
  });
  if (!res.ok) {
    error(res.status, res.error);
  }
  return { book: res.data };
};
