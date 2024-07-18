import { fetchBackend } from "$lib/backend";
import { error, fail } from "@sveltejs/kit";
import type { Actions, PageServerLoad } from "./$types";
import type { Comment, Post, ReactionType } from "$lib/types";
import { z } from "zod";
import { reactionTypeSchema } from "$lib/schemas";

const postIdSchema = z
  .number({ coerce: true, message: "Post ID is not a number!" })
  .int("Post ID must be an integer!")
  .safe("Post ID must be within safe range!");

const commentSchema = z.object({
  post_id: postIdSchema,
  parent_id: z
    .number({ coerce: true, message: "Parent ID is not a number!" })
    .int("Parent ID must be an integer!")
    .safe("Parent ID must be within safe range!")
    .nullable(),
  content: z.string().min(1, "Your comment must not be empty!"),
});

const reactionSchema = z.object({
  for_type: z.union([z.literal("post"), z.literal("comment")], { message: "Reaction must be for a post or a comment!" }),
  post_id: postIdSchema,
  reaction_type: reactionTypeSchema,
});

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
  async react({ cookies, fetch, request, setHeaders }) {
    const formData = await request.formData();

    const data = await reactionSchema.spa({
      post_id: formData.get("forId"),
      for_type: formData.get("forType"),
      reaction_type: formData.get("reactionType"),
    });

    if (!data.success) {
      return fail(400, { validationError: data.error.flatten().fieldErrors });
    }

    const res = await fetchBackend<{ reaction_type: ReactionType }>("/reactions/create", {
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

    return { reactionType: res.data.reaction_type };
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
  return {
    post: res.data.post,
    comments: res.data.comments,
  };
};
