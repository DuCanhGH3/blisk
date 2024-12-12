import { fail, redirect } from "@sveltejs/kit";
import { z } from "zod";
import { fetchBackend } from "$lib/backend";
import type { Actions, PageServerLoad } from "./$types";
import { convertFormData, safeRedirect } from "$lib/utils";

const loginSchema = z.object({
  username: z.string().min(1, "Please enter a valid username!"),
  password: z.string().min(1, "Please enter a valid password!"),
});

export const load: PageServerLoad = ({ locals, url }) => {
  if (locals.user) {
    redirect(303, safeRedirect(url.searchParams.get("redirectTo")));
  }
  return {
    title: "Login",
  };
};

export const actions: Actions = {
  async login(event) {
    try {
      const data = await loginSchema.spa(convertFormData(await event.request.formData()));
      if (!data.success) {
        return fail(400, { validationError: data.error.flatten().fieldErrors });
      }
      const res = await fetchBackend<{ token: string; expires_in: number }>("/auth/login", {
        authz: false,
        type: "url-encoded",
        event,
        method: "POST",
        body: new URLSearchParams(data.data),
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
      event.cookies.set("token", res.data.token, cookiesOptions);
    } catch (err) {
      console.error(err);
      return fail(500, { error: "Internal Server Error" });
    }
    redirect(303, safeRedirect(event.url.searchParams.get("redirectTo")));
  },
  logout({ cookies, locals, url }) {
    const cookiesOptions = {
      httpOnly: true,
      secure: true,
      sameSite: "strict",
      path: "/",
    } as const;
    cookies.delete("token", cookiesOptions);
    locals.user = null;
    redirect(302, safeRedirect(url.searchParams.get("redirectTo")));
  }
};
