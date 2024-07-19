import { fail, redirect } from "@sveltejs/kit";

import { base } from "$app/paths";

import type { Actions, PageServerLoad } from "./$types";
import { z } from "zod";
import { fetchBackend } from "$lib/backend";

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
  async login({ cookies, fetch, request, setHeaders }) {
    try {
      const formData = await request.formData();
      const data = await loginSchema.spa({
        username: formData.get("username"),
        password: formData.get("password"),
      });
      if (!data.success) {
        return fail(400, { validationError: data.error.flatten().fieldErrors });
      }
      const res = await fetchBackend<{ token_type: string; expires_in: number; id_token: string }>("/auth/login?client_id=abc", {
        authz: false,
        cookies,
        fetch,
        setHeaders,
        method: "POST",
        body: JSON.stringify(data.data),
        signal: AbortSignal.timeout(10000),
      });
      if (!res.ok) {
        return fail(res.status, { error: res.error });
      }
      const cookiesOptions = {
        httpOnly: true,
        secure: true,
        sameSite: "strict",
        path: "/",
        maxAge: res.data.expires_in,
      } as const;
      cookies.set("token_type", res.data.token_type, cookiesOptions);
      cookies.set("token", res.data.id_token, cookiesOptions);
    } catch (err) {
      console.error(err);
      return fail(500, { error: "Internal Server Error" });
    }
  },
  async logout({ cookies, locals }) {
    try {
      const cookiesOptions = {
        httpOnly: true,
        secure: true,
        sameSite: "strict",
        path: "/",
      } as const;
      cookies.delete("token_type", cookiesOptions);
      cookies.delete("token", cookiesOptions);
      locals.user = null;
    } catch (err) {
      console.error(err);
      return fail(500, { error: "Internal Server Error" });
    }
  },
};
