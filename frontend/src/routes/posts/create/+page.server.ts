import { z } from "zod";
import type { Actions } from "./$types";
import { error, fail, redirect } from "@sveltejs/kit";
import { fetchBackend } from "$lib/backend";
import { base } from "$app/paths";
import { convertFormData, safeRedirect } from "$lib/utils";
import type { UserMetadata } from "$lib/types";

const postSchema = z.object({
  title: z.string().min(1, "Title must not be empty!").max(500, "Title is too long!"),
  content: z.string().min(1, "Content must not be empty!"),
  book: z.string().min(1, "You must point to a book!"),
  reaction: z.union([z.literal("like"), z.literal("dislike")], { message: "Reaction is not valid!" }),
  images: z.array(z.instanceof(File, { message: "One of the images is not valid!" })),
});

export const actions: Actions = {
  async default(event) {
    const data = await postSchema.spa(convertFormData(await event.request.formData()));
    if (!data.success) {
      return fail(400, { validationError: data.error.flatten().fieldErrors });
    }
    const backendResponse = await fetchBackend<{ id: number }>("/posts", {
      authz: true,
      event,
      method: "POST",
      body: JSON.stringify(data.data),
    });
    if (!backendResponse.ok) {
      return fail(backendResponse.status, { error: backendResponse.error });
    }
    redirect(307, safeRedirect(event.url.searchParams.get("redirectTo"), `${base}/posts/${backendResponse.data.id}`));
  },
};

export const load = async (event) => {
  if (!event.locals.user) {
    redirect(307, safeRedirect(event.url.searchParams.get("redirectTo"), `${base}/`));
  }
  const metadata = await fetchBackend<UserMetadata>(`/users/${event.locals.user.name}/metadata`, {
    authz: "optional",
    event,
  });
  if (!metadata.ok) {
    error(metadata.status, metadata.error);
  }
  return {
    title: "Create a Post",
    books: metadata.data.books,
  };
};
