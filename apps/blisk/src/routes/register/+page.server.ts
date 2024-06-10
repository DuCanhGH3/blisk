import { fail, isRedirect, redirect } from "@sveltejs/kit";
import { z } from "zod";

import { base } from "$app/paths";

import type { Actions, PageServerLoad } from "./$types";
import { BACKEND_URL } from "$env/static/private";
import { handleBackendError } from "$lib/backendResponse";

const registerSchema = z.object({
  username: z.string().min(1, "Please enter a valid username!"),
  email: z.string().email("Please enter a valid email!"),
  password: z.string().min(6, "Please enter a valid password of at least 6 characters!"),
});

export const load: PageServerLoad = ({ locals }) => {
  if (locals.user) {
    redirect(303, `${base}/`);
  }
  return {
    title: "Register",
  };
};

export const actions: Actions = {
  async register({ cookies, fetch, locals, request }) {
    try {
      const formData = await request.formData();
      const data = await registerSchema.spa({
        username: formData.get("username"),
        email: formData.get("email"),
        password: formData.get("password"),
      });
      if (!data.success) {
        return fail(400, { validationError: data.error.flatten().fieldErrors });
      }
      const res = await fetch(`${BACKEND_URL}/users/register`, {
        method: "POST",
        body: JSON.stringify(data.data),
        headers: {
          "Content-Type": "application/json",
          Accept: "application/json",
        },
        signal: AbortSignal.timeout(10000),
      });
      if (!res.ok) {
        return await handleBackendError(res);
      }
      redirect(307, `${base}/login`);
    } catch (err) {
      if (isRedirect(err)) {
        throw err;
      }
      console.log(err);
      if (err instanceof Error && err.name === "TimeoutError") {
        return fail(500, { error: "Server is currently under heavy load." });
      }
      return fail(500, { error: "Internal Server Error" });
    }
  },
};
