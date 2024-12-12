import { fetchBackend } from "$lib/backend";
import type { BookCategoryWithBooks } from "$lib/types.js";
import { error } from "@sveltejs/kit";

export const load = async (event) => {
  const data = await fetchBackend<BookCategoryWithBooks[]>("/books/categories", {
    authz: false,
    event,
  });
  if (!data.ok) {
    error(data.status, data.error);
  }
  return { title: "Home", categories: data.data };
};
