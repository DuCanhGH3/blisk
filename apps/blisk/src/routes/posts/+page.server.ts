import { z } from "zod";
import type { Actions } from "./$types";
import { fail, redirect } from "@sveltejs/kit";
import { fetchBackend } from "$lib/backend";
import { base } from "$app/paths";

const postSchema = z.object({
  title: z.string().min(1, "Title must not be empty!"),
  content: z.string().min(1, "Content must not be empty!"),
});

export const actions: Actions = {
  async default({ cookies, fetch, setHeaders, request }) {
    const formData = await request.formData();
    const data = await postSchema.spa({
      title: formData.get("title"),
      content: formData.get("content"),
    });
    if (!data.success) {
      return fail(400, { validationError: data.error.flatten().fieldErrors });
    }
    const backendResponse = await fetchBackend<{ id: number }>("/posts/create", {
      authz: true,
      cookies,
      fetch,
      setHeaders,
      method: "POST",
      body: JSON.stringify({
        title: data.data.title,
        content: data.data.content,
      }),
    });
    if (!backendResponse.ok) {
      return fail(backendResponse.status, { error: backendResponse.error });
    }
    redirect(307, `${base}/posts/${backendResponse.data.id}`);
  },
};
