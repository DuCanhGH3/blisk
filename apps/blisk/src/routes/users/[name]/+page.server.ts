import { createReaction, fetchBackend } from "$lib/backend";
import { error } from "@sveltejs/kit";
import type { Actions, PageServerLoad } from "./$types";
import type { Post } from "$lib/types";

interface LoadData {
  name: string;
  posts: Post[];
}

export const actions: Actions = {
  async react({ cookies, fetch, request, setHeaders }) {
    return await createReaction(await request.formData(), fetch, cookies, setHeaders);
  },
};

export const load: PageServerLoad = async ({ cookies, fetch, params, setHeaders }) => {
  const user = await fetchBackend<LoadData>(`/users?username=${params.name}`, {
    authz: "optional",
    cookies,
    fetch,
    setHeaders,
  });
  if (!user.ok) {
    error(user.status, user.error);
  }
  return { title: `User ${user.data.name}`, data: user.data };
};
