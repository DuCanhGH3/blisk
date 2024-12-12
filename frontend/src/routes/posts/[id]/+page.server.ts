import { error } from "@sveltejs/kit";
import { createComment, createReaction, editComment, fetchBackend } from "$lib/backend";
import type { Post, RequireFields } from "$lib/types";
import type { Actions, PageServerLoad } from "./$types";

export const actions: Actions = {
  async react(event) {
    return await createReaction(event);
  },
  async comment(event) {
    return await createComment(event.params.id, event.url.searchParams.get("parentId"), event);
  },
  async editComment(event) {
    return await editComment(event);
  },
};

export const load: PageServerLoad = async (event) => {
  const res = await fetchBackend<RequireFields<Post, "comments">>(`/posts/${event.params.id}`, {
    authz: "optional",
    event,
    signal: AbortSignal.timeout(2000),
  });
  if (!res.ok) {
    error(res.status, { message: res.error });
  }
  return { title: `${res.data.title.slice(0, 50)}`, post: res.data };
};
