import { z } from "zod";
import type { Actions } from "./$types";
import { fail } from "@sveltejs/kit";
import { fetchBackend } from "$lib/backend";

const postSchema = z.object({
  title: z.string().min(1, "Title must not be empty!"),
  summary: z.string().min(1, "Summary must not be empty!"),
});

export const actions: Actions = {
  async default({ cookies, fetch, setHeaders, request }) {
    const formData = await request.formData();
    const data = await postSchema.spa({
      title: formData.get("title"),
      summary: formData.get("summary"),
    });
    if (!data.success) {
      return fail(400, { validationError: data.error.flatten().fieldErrors });
    }
    const backendResponse = await fetchBackend<{ id: number }>("/books/create", {
      authz: true,
      cookies,
      fetch,
      setHeaders,
      method: "POST",
      body: formData,
    });
    if (!backendResponse.ok) {
      return fail(backendResponse.status, { error: backendResponse.error });
    }
    // redirect(307, `${base}/posts/${backendResponse.data.id}`);
  },
};
