import { error, fail } from "@sveltejs/kit";
import { z } from "zod";
import { fetchBackend } from "$lib/backend";
import type { PageServerLoad } from "./$types.js";
import type { BookMetadata, BooksMetadata } from "$lib/types.js";

const uploadSchema = z.object({
  volume: z.string({ message: "Invalid volume!" }).refine((value) => !Number.isNaN(Number(value)), { message: "Invalid volume!" }),
  language: z.string({ message: "Invalid language!" }).min(1, "Invalid language!"),
  name: z.string({ message: "Invalid name!" }).min(1, "A name for this chapter please!"),
  content: z.string({ message: "Invalid content!" }).min(150, "Some content, no?"),
});

export const actions = {
  async default(event) {
    const formData = await event.request.formData();
    const data = await uploadSchema.spa({
      volume: formData.get("volume"),
      language: formData.get("language"),
      name: formData.get("name"),
      content: formData.get("content"),
    });
    if (!data.success) {
      return fail(400, { validationError: data.error.flatten().fieldErrors });
    }
    const res = await fetchBackend("/books/chapters", {
      authz: true,
      type: "url-encoded",
      event,
      noSuccessContent: true,
      method: "POST",
      body: new URLSearchParams(data.data),
    });
    if (!res.ok) {
      return fail(res.status, { error: res.error });
    }
  },
};

export const load: PageServerLoad = async (event) => {
  const [booksMetadata, bookMetadata] = await Promise.all([
    fetchBackend<BooksMetadata>("/books/metadata", {
      authz: false,
      event,
    }),
    fetchBackend<BookMetadata>(`/books/${event.params.id}/metadata`, {
      authz: false,
      event,
    }),
  ]);
  if (!booksMetadata.ok) {
    error(booksMetadata.status, booksMetadata.error);
  }
  if (!bookMetadata.ok) {
    error(bookMetadata.status, bookMetadata.error);
  }
  return { languages: booksMetadata.data.languages, metadata: bookMetadata.data };
};
