import { error } from "@sveltejs/kit";
import { createReaction, editComment, fetchBackend } from "$lib/backend";
import type { Actions, PageServerLoad } from "./$types";
import type { Comment } from "$lib/types";

export const actions: Actions = {
  async react(event) {
    return await createReaction(event);
  },
  async editComment(event) {
    return await editComment(event);
  },
};

export const load: PageServerLoad = async (event) => {
  const user = await fetchBackend<Comment[]>(`/comments?user=${event.params.name}`, {
    authz: "optional",
    event,
  });
  if (!user.ok) {
    error(user.status, user.error);
  }
  return { title: `${event.params.name}'s comments`, comments: user.data };
};
