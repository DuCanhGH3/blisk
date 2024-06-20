import { fetchBackend } from "$lib/backend";
import { error } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";
import type { Post } from "$lib/types";

interface Book {
  title: string;
  summary: string;
  reviews: Post[];
}

export const load: PageServerLoad = async ({ cookies, fetch, setHeaders }) => {
  const res = await fetchBackend<Book[]>("/books/read", {
    authz: false,
    cookies,
    fetch,
    setHeaders,
  });
  if (!res.ok) {
    error(res.status, res.error);
  }
  return { books: res.data };
};
