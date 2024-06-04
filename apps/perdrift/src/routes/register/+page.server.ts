import { fail, redirect } from "@sveltejs/kit";

import { base } from "$app/paths";

import type { Actions, PageServerLoad } from "./$types";
import { hashPassword, saveUser } from "$lib/users";

export const load: PageServerLoad = ({ locals }) => {
  if (locals.user) {
    redirect(303, `${base}/`);
  }
  return {
    title: "Register",
  };
};

export const actions: Actions = {
  async register({ cookies, locals, request }) {
    try {
      const formData = await request.formData();
      const username = formData.get("username");
      const email = formData.get("email");
      const password = formData.get("password");
      if (typeof username !== "string") {
        return fail(400, { usernameError: "Username is not valid!" });
      }
      if (typeof email !== "string") {
        return fail(400, { emailError: "Email is not valid!" });
      }
      if (typeof password !== "string") {
        return fail(400, { passwordError: "Password is not valid!" });
      }
      const existingUser = await locals.prisma.user.findFirst({
        where: {
          OR: [{ email: email }, { name: username }],
        },
        select: {
          email: true,
          password: true,
        },
      });
      if (existingUser !== null) {
        return fail(401, { error: "This account already exists." });
      }
      const user = await locals.prisma.user.create({
        data: {
          name: username,
          email: email,
          password: await hashPassword(password),
        },
        select: {
          id: true,
        },
      });
      saveUser({ id: user.id, cookies });
    } catch (err) {
      console.log(err);
      if (err instanceof Error && err.name === "TimeoutError") {
        return fail(500, { error: "Server is currently under heavy load." });
      }
      return fail(500, { error: "Internal Server Error" });
    }
  },
};
