import { fail, redirect } from "@sveltejs/kit";
import { z } from "zod";
import { base } from "$app/paths";
import { fetchBackend } from "$lib/backend";
import type { Actions, PageServerLoad } from "./$types";

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
  async default({ cookies, fetch, request, setHeaders }) {
    const formData = await request.formData();
    const data = await registerSchema.spa({
      username: formData.get("username"),
      email: formData.get("email"),
      password: formData.get("password"),
    });
    if (!data.success) {
      return fail(400, { validationError: data.error.flatten().fieldErrors });
    }
    const res = await fetchBackend("/auth/register", {
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
    redirect(307, `${base}/auth/login`);
  },
};
