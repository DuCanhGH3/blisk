import { z } from "zod";
import type { Actions, PageServerLoad } from "./$types";
import { error, fail } from "@sveltejs/kit";
import { fetchBackend } from "$lib/backend";
import type { BookCategory } from "$lib/types";
import { bookCategoryIdSchema } from "$lib/schemas";

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
});

export const actions: Actions = {
  async default({ cookies, fetch, setHeaders, request }) {
    const formData = await request.formData();
    const data = await createSchema.spa({
      title: formData.get("title"),
      slug: formData.get("slug"),
      pages: formData.get("pages"),
      summary: formData.get("summary"),
      categories: formData.getAll("categories"),
    });
    if (!data.success) {
      return fail(400, { validationError: data.error.flatten().fieldErrors });
    }
    const backendResponse = await fetchBackend<{ id: number }>("/books", {
      authz: true,
      cookies,
      fetch,
      setHeaders,
      method: "POST",
      body: JSON.stringify({
        ...data.data,
        language: "en-US",
      }),
    });
    if (!backendResponse.ok) {
      return fail(backendResponse.status, { error: backendResponse.error });
    }
    // redirect(307, `${base}/posts/${backendResponse.data.id}`);
  },
};

export const load: PageServerLoad = async ({ cookies, fetch, setHeaders }) => {
  const res = await fetchBackend<BookCategory[]>(`/books/categories`, {
    authz: false,
    cookies,
    fetch,
    setHeaders,
  });
  if (!res.ok) {
    error(res.status, res.error);
  }
  return { categories: res.data };
};
