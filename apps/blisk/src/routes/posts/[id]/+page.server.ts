import { fetchBackend } from "$lib/backend";
import { error, fail } from "@sveltejs/kit";
import type { Actions, PageServerLoad } from "./$types";
import type { Comment, Post } from "$lib/types";
import { z } from "zod";

const commentSchema = z.object({
  post_id: z.number().int().safe(),
  parent_id: z.coerce.number().int().nullable(),
  content: z.string().min(1, "Your comment must not be empty!"),
});

export const actions: Actions = {
  async comment({ cookies, fetch, params, request, setHeaders, url }) {
    const formData = await request.formData();

    const data = await commentSchema.spa({
      post_id: Number.parseInt(params.id),
      parent_id: url.searchParams.get("parentId"),
      content: formData.get("content"),
    });

    if (!data.success) {
      return fail(400, { validationError: data.error.flatten().fieldErrors });
    }

    const res = await fetchBackend<{ id: number }>("/comments/create", {
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
};

export const load: PageServerLoad = async ({ cookies, fetch, params, setHeaders }) => {
  const res = await fetchBackend<{ post: Post; comments: Comment[] }>(`/posts/read?post_id=${params.id}`, {
    authz: false,
    cookies,
    fetch,
    setHeaders,
    signal: AbortSignal.timeout(2000),
  });
  if (!res.ok) {
    error(res.status, { message: res.error });
  }
  console.log(res.data.comments);
  return {
    post: res.data.post,
    comments: res.data.comments,
  };
};
