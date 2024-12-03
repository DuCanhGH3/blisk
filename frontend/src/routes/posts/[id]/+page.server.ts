import { error } from "@sveltejs/kit";
import { createComment, createReaction, editComment, fetchBackend } from "$lib/backend";
import type { Post, RequireFields } from "$lib/types";
import type { Actions, PageServerLoad } from "./$types";

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

export const load: PageServerLoad = async ({ cookies, fetch, params, setHeaders }) => {
  const res = await fetchBackend<RequireFields<Post, "comments">>(`/posts/${params.id}`, {
    authz: "optional",
    cookies,
    fetch,
    setHeaders,
    signal: AbortSignal.timeout(2000),
  });
  if (!res.ok) {
    error(res.status, { message: res.error });
  }
  return { title: `${res.data.title.slice(0, 50)}`, post: res.data };
};
