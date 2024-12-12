import { fetchBackend } from "$lib/backend";
import { error } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";
import type { Book } from "$lib/types";

export const load: PageServerLoad = async (event) => {
  const q = event.url.searchParams.get("q");
  const query = q ? `&q=${q}` : "";
  const authors = event.url.searchParams
    .getAll("author")
    .map((author) => `&authors=${author}`)
    .join("");
  const categories = event.url.searchParams
    .getAll("category")
    .map((author) => `&categories=${author}`)
    .join("");
  const fetchUrl: `/${string}` = `/books?include_reviews=false${query}${authors}${categories}`;
  const res = await fetchBackend<Book[]>(fetchUrl, {
    authz: "optional",
    event,
  });
  if (!res.ok) {
    error(res.status, res.error);
  }
  return { title: q ? `${q} - Books search` : "Books", books: res.data };
};
