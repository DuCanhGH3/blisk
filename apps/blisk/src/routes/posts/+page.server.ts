import { createReaction, fetchBackend } from "$lib/backend";
import type { Post } from "$lib/types.js";
import { error } from "@sveltejs/kit"

export const actions = {
  async react({ cookies, fetch, request, setHeaders }) {
    return await createReaction(await request.formData(), fetch, cookies, setHeaders);
  },
};

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
  return { title: "Posts", posts: data.data };
};
