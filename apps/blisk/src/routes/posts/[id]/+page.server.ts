import { fetchBackend } from "$lib/backend";
import { error } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";
import type { Post } from "$lib/types";

export const load: PageServerLoad = async ({ cookies, fetch, params, setHeaders }) => {
  const backendResponse = await fetchBackend<Post>(`/posts/read?post_id=${params.id}`, {
    authz: false,
    cookies,
    fetch,
    setHeaders,
  });
  if (!backendResponse.ok) {
    error(backendResponse.status, { message: backendResponse.error });
  }
  return {
    post: backendResponse.data,
  };
};
