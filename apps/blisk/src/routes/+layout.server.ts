import type { User } from "$lib/types.js";

export const load = ({ locals }) => ({
  user: locals.user ? ({ name: locals.user.name } satisfies Pick<User, "name">) : null,
});
