import { fail, redirect } from "@sveltejs/kit";

import { base } from "$app/paths";

import type { Actions, PageServerLoad } from "./$types";
import { z } from "zod";
import { BACKEND_URL } from "$env/static/private";
import { handleBackendError } from "$lib/backendResponse";

const loginSchema = z.object({
  username: z.string().min(1, "Please enter a valid username!"),
  password: z.string().min(1, "Please enter a valid password!"),
});

export const load: PageServerLoad = ({ locals }) => {
  if (locals.user) {
    redirect(303, `${base}/`);
  }
  return {
    title: "Login",
  };
};

export const actions: Actions = {
  async login({ cookies, fetch, request }) {
    try {
      const formData = await request.formData();
      const data = await loginSchema.spa({
        username: formData.get("username"),
        password: formData.get("password"),
      });
      if (!data.success) {
        return fail(400, { validationError: data.error.flatten().fieldErrors });
      }
      const res = await fetch(`${BACKEND_URL}/users/login?client_id=abc`, {
        method: "POST",
        body: JSON.stringify(data.data),
        headers: {
          Accept: "application/json",
          "Content-Type": "application/json",
        },
        signal: AbortSignal.timeout(10000),
      });
      if (!res.ok) {
        return await handleBackendError(res);
      }
      const loginInfo = await res.json();
      const cookiesOptions = {
        httpOnly: true,
        secure: true,
        sameSite: "strict",
        path: "/",
        maxAge: loginInfo.expires_in,
      } as const;
      cookies.set("token_type", loginInfo.token_type, cookiesOptions);
      cookies.set("token", loginInfo.id_token, cookiesOptions);
    } catch (err) {
      console.log(err);
      if (err instanceof Error && err.name === "TimeoutError") {
        return fail(500, { error: "Server is currently under heavy load." });
      }
      return fail(500, { error: "Internal Server Error" });
    }
  },
  async logout({ locals }) {
    try {
      locals.user = null;
    } catch (err) {
      if (err instanceof Error && err.name === "TimeoutError") {
        return fail(500, { error: "Server is currently under heavy load." });
      }
      return fail(500, { error: "Internal Server Error" });
    }
  },
};