import { error } from "@sveltejs/kit";
import { createReaction, editComment, fetchBackend } from "$lib/backend";
import type { Actions, PageServerLoad  } from "./$types";
import type { Comment } from "$lib/types";

export const actions: Actions = {
  async react({ cookies, fetch, request, setHeaders }) {
    return await createReaction(await request.formData(), fetch, cookies, setHeaders);
  },
  async editComment({ cookies, fetch, request, setHeaders }) {
    return await editComment(await request.formData(), fetch, cookies, setHeaders);
  },
};

export const load: PageServerLoad = async ({ cookies, fetch, params, setHeaders }) => {
  const user = await fetchBackend<Comment[]>(`/users/${params.name}/comments`, {
    authz: "optional",
    cookies,
    fetch,
    setHeaders,
  });
  if (!user.ok) {
    error(user.status, user.error);
  }
  return { title: `${params.name}'s comments`, comments: user.data };
};
