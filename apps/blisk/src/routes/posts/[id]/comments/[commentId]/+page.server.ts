import type { Post, RequireFields } from "$lib/types.js";
import { error } from "@sveltejs/kit";
import { createComment, createReaction, editComment, fetchBackend } from "$lib/backend";
import type { Actions } from "./$types";

export const actions: Actions = {
  async react({ cookies, fetch, request, setHeaders }) {
    return await createReaction(await request.formData(), fetch, cookies, setHeaders);
  },
  async comment({ cookies, fetch, params, request, setHeaders, url }) {
    return await createComment(params.id, url.searchParams.get("parentId"), await request.formData(), fetch, cookies, setHeaders);
  },
  async editComment({ cookies, fetch, request, setHeaders }) {
    return await editComment(await request.formData(), fetch, cookies, setHeaders);
  },
};

export const load = async ({ cookies, fetch, params, setHeaders }) => {
  const res = await fetchBackend<RequireFields<Post, "comments">>(`/posts?post_id=${params.id}&comment_id=${params.commentId}`, {
    authz: "optional",
    cookies,
    fetch,
    setHeaders,
    signal: AbortSignal.timeout(2000),
  });
  if (!res.ok) {
    error(res.status, { message: res.error });
  }
  return { post: res.data };
};
