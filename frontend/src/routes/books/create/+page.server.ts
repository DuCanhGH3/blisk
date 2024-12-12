import { z } from "zod";
import type { Actions, PageServerLoad } from "./$types";
import { error, fail, redirect } from "@sveltejs/kit";
import { fetchBackend } from "$lib/backend";
import type { BooksMetadata } from "$lib/types";
import { bookCategoryIdSchema } from "$lib/schemas";
import { base } from "$app/paths";
import { convertFormData } from "$lib/utils";

const createSchema = z.object({
  title: z.string().min(1, "Title must not be empty!"),
  slug: z
    .string()
    .min(1, "Slug must not be empty!")
    .refine(
      (value) => {
        return /^[a-z0-9](-?[a-z0-9])*$/.test(value);
      },
      { message: "Slug is not valid! It must only contain ASCII characters, numbers, and/or hyphens." }
    ),
  pages: z.number({ coerce: true, message: "Number of pages must be a number!" }).safe().int("Number of pages must be an integer!"),
  summary: z.string().min(1, "Synopsis must not be empty!"),
  categories: z.array(bookCategoryIdSchema),
  cover_image: z.instanceof(File, { message: "You must select a valid file!" }),
  spine_image: z.instanceof(File, { message: "You must select a valid file!" }),
});

export const actions: Actions = {
  async default(event) {
    const formData = await event.request.formData();
    const data = await createSchema.spa(convertFormData(formData));
    if (!data.success) {
      return fail(400, { validationError: data.error.flatten().fieldErrors });
    }
    formData.set("language", "en-US");
    formData.append("authors", "1");
    const backendResponse = await fetchBackend("/books", {
      authz: true,
      type: "multipart",
      event,
      noSuccessContent: true,
      method: "POST",
      body: formData,
    });
    if (!backendResponse.ok) {
      return fail(backendResponse.status, { error: backendResponse.error });
    }
    redirect(307, `${base}/books/${data.data.slug}`);
  },
};

export const load: PageServerLoad = async (event) => {
  const res = await fetchBackend<BooksMetadata>("/books/metadata", {
    authz: false,
    event,
  });
  if (!res.ok) {
    error(res.status, res.error);
  }
  return { authors: res.data.authors, categories: res.data.categories };
};
