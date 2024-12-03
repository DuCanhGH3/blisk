import { createReaction, fetchBackend } from "$lib/backend";
import { error } from "@sveltejs/kit";
import type { Actions, PageServerLoad } from "./$types";
import type { Book } from "$lib/types";

export const actions: Actions = {
  async react({ cookies, fetch, request, setHeaders }) {
    return await createReaction(await request.formData(), fetch, cookies, setHeaders);
  },
};

export const load: PageServerLoad = async ({ cookies, params, setHeaders }) => {
  const res = await fetchBackend<Book>(`/books/${params.id}`, {
    authz: "optional",
    cookies,
    fetch,
    setHeaders,
  });
  if (!res.ok) {
    error(res.status, res.error);
  }
  return { book: res.data };
};
