import { fetchBackend } from "$lib/backend";
import { error } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";
import type { Book } from "$lib/types";

export const load: PageServerLoad = async ({ cookies, fetch, setHeaders, url }) => {
  const query = url.searchParams.get("q");
  const fetchUrl: `/${string}` = query ? `/books?q=${query}&offset=0&include_reviews=false` : "/books";
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
