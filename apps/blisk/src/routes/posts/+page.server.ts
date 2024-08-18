import { fetchBackend } from "$lib/backend";
import type { Post } from "$lib/types.js";
import { error } from "@sveltejs/kit";

export const load = async ({ cookies, fetch, setHeaders }) => {
  const data = await fetchBackend<Post[]>("/posts", {
    authz: "optional",
    cookies,
    fetch,
    setHeaders,
  });
  if (!data.ok) {
    error(data.status, data.error);
  }
  return { posts: data.data };
};
