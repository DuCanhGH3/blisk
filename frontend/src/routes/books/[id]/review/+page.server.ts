import { createReaction } from "$lib/backend";
import type { Actions } from "./$types";

export const actions: Actions = {
  async react(event) {
    return await createReaction(event);
  },
};
