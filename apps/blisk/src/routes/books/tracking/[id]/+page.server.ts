import { fetchBackend } from "$lib/backend";
import type { ReadingTracker } from "$lib/types.js";
import { error } from "@sveltejs/kit";

export const load = async ({ params, cookies, fetch, setHeaders }) => {
  const res = await fetchBackend<ReadingTracker>(`/books/tracker/${params.id}`, {
    authz: true,
    cookies,
    fetch,
    setHeaders,
  });

  if (!res.ok) {
    error(res.status, res.error);
  }

  return { data: res.data };
};
