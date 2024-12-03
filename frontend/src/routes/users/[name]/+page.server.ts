import { createReaction, fetchBackend } from "$lib/backend";
import type { Post } from "$lib/types";
import { error } from "@sveltejs/kit";
import type { Actions, PageServerLoad } from "./$types";

export const actions: Actions = {
  async react({ cookies, fetch, request, setHeaders }) {
    return await createReaction(await request.formData(), fetch, cookies, setHeaders);
  },
};

export const load: PageServerLoad = async ({ cookies, fetch, params, setHeaders }) => {
  const user = await fetchBackend<Post[]>(`/posts?user=${params.name}`, {
    authz: "optional",
    cookies,
    fetch,
    setHeaders,
  });
  if (!user.ok) {
    error(user.status, user.error);
  }
  return { title: `${params.name}'s posts`, posts: user.data };
};
