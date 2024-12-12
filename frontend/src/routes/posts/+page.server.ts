import { createReaction, fetchBackend } from "$lib/backend";
import type { Post } from "$lib/types.js";
import { error } from "@sveltejs/kit";

export const actions = {
  async react(event) {
    return await createReaction(event);
  },
};

export const load = async (event) => {
  const data = await fetchBackend<Post[]>("/posts", {
    authz: "optional",
    event,
  });
  if (!data.ok) {
    error(data.status, data.error);
  }
  return { title: "Posts", posts: data.data };
};
