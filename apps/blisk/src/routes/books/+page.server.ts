import { fetchBackend } from "$lib/backend";
import { error } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";
import type { Book } from "$lib/types";

export const load: PageServerLoad = async ({ cookies, fetch, setHeaders, url }) => {
  const q = url.searchParams.get("q");
  const query = q ? `&q=${q}` : "";
  const authors = url.searchParams
    .getAll("author")
    .map((author) => `&authors=${author}`)
    .join("");
  const categories = url.searchParams
    .getAll("category")
    .map((author) => `&categories=${author}`)
    .join("");
  const fetchUrl: `/${string}` = `/books?include_reviews=false${query}${authors}${categories}`;
  const res = await fetchBackend<Book[]>(fetchUrl, {
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
