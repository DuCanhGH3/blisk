import { createReaction, fetchBackend } from "$lib/backend";
import type { Post } from "$lib/types";
import { error } from "@sveltejs/kit";
import type { Actions, PageServerLoad } from "./$types";

export const actions: Actions = {
  async react(event) {
    return await createReaction(event);
  },
};

export const load: PageServerLoad = async (event) => {
  const user = await fetchBackend<Post[]>(`/posts?user=${event.params.name}`, {
    authz: "optional",
    event,
  });
  if (!user.ok) {
    error(user.status, user.error);
  }
  return { title: `${event.params.name}'s posts`, posts: user.data };
};
