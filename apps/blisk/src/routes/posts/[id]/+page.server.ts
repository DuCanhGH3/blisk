import { error, fail } from "@sveltejs/kit";
import { createReaction, fetchBackend } from "$lib/backend";
import type { Comment, Post } from "$lib/types";
import type { Actions, PageServerLoad } from "./$types";
import { commentSchema } from "$lib/schemas";

export const actions: Actions = {
  async comment({ cookies, fetch, params, request, setHeaders, url }) {
    const formData = await request.formData();

    const data = await commentSchema.spa({
      post_id: params.id,
      parent_id: url.searchParams.get("parentId"),
      content: formData.get("content"),
    });

    if (!data.success) {
      return fail(400, { validationError: data.error.flatten().fieldErrors });
    }

    const res = await fetchBackend<{ id: number }>("/comments", {
      authz: true,
      cookies,
      fetch,
      setHeaders,
      method: "POST",
      body: JSON.stringify(data.data),
      signal: AbortSignal.timeout(10000),
    });

    if (!res.ok) {
      return fail(res.status, { error: res.error });
    }

    return { id: res.data.id };
  },
  async react({ cookies, fetch, request, setHeaders }) {
    return await createReaction(await request.formData(), fetch, cookies, setHeaders);
  },
};

export const load: PageServerLoad = async ({ cookies, fetch, params, setHeaders }) => {
  const res = await fetchBackend<{ post: Post; comments: Comment[] }>(`/posts?post_id=${params.id}`, {
    authz: "optional",
    cookies,
    fetch,
    setHeaders,
    signal: AbortSignal.timeout(2000),
  });
  if (!res.ok) {
    error(res.status, { message: res.error });
  }
  return {
    post: res.data.post,
    comments: res.data.comments,
  };
};
