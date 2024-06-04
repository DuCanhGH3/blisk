import { fail, redirect } from "@sveltejs/kit";

import { base } from "$app/paths";

import type { Actions, PageServerLoad } from "./$types";
import { clearUser, saveUser, verifyPassword } from "$lib/users";

export const load: PageServerLoad = ({ locals }) => {
  if (locals.user) {
    redirect(303, `${base}/`);
  }
  return {
    title: "Login",
  };
};

export const actions: Actions = {
  async login({ cookies, locals, request }) {
    try {
      const formData = await request.formData();
      const username = formData.get("username");
      const password = formData.get("password");
      if (typeof username !== "string") {
        return fail(400, { usernameError: "Username is not valid!" });
      }
      if (typeof password !== "string") {
        return fail(400, { passwordError: "Password is not valid!" });
      }
      const user = await locals.prisma.user.findFirst({
        where: {
          OR: [{ email: username }, { name: username }],
        },
        select: {
          id: true,
          name: true,
          email: true,
          password: true,
        },
      });
      if (user === null) {
        return fail(401, { error: "This account does not exist." });
      }
      if (!(await verifyPassword(user.password, password))) {
        return fail(401, { error: "Password is incorrect!" });
      }
      saveUser({ id: user.id, cookies });
    } catch (err) {
      console.log(err);
      if (err instanceof Error && err.name === "TimeoutError") {
        return fail(500, { error: "Server is currently under heavy load." });
      }
      return fail(500, { error: "Internal Server Error" });
    }
  },
  async logout({ cookies, locals }) {
    try {
      clearUser({ cookies });
      locals.user = null;
    } catch (err) {
      if (err instanceof Error && err.name === "TimeoutError") {
        return fail(500, { error: "Server is currently under heavy load." });
      }
      return fail(500, { error: "Internal Server Error" });
    }
  },
};
