import { createReaction } from "$lib/backend";
import type { Actions } from "./$types";

export const actions: Actions = {
  async react({ cookies, fetch, request, setHeaders }) {
    return await createReaction(await request.formData(), fetch, cookies, setHeaders);
  },
};
