import { z } from "zod";
import type { Actions } from "./$types";
import { fail } from "@sveltejs/kit";
import { BACKEND_URL } from "$env/static/private";
import { handleBackendError } from "$lib/backendResponse";

const postSchema = z.object({
  title: z.string(),
  content: z.string(),
});

export const actions: Actions = {
  async default({ fetch, request }) {
    const formData = await request.formData();
    const data = await postSchema.spa({
      title: formData.get("title"),
      content: formData.get("content"),
    });
    if (!data.success) {
      return fail(400, { validationError: data.error.flatten().fieldErrors });
    }
    const backendResponse = await fetch(`${BACKEND_URL}/posts/create`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        user_id: "1",
        title: data.data.title,
        content: data.data.content,
      }),
    });
    if (!backendResponse.ok) {
      return handleBackendError(backendResponse);
    }
  },
};
