import { fail, redirect } from "@sveltejs/kit";
import { z } from "zod";
import { base } from "$app/paths";
import { fetchBackend } from "$lib/backend";
import type { Actions, PageServerLoad } from "./$types";

const registerSchema = z.object({
  username: z.string().min(1, "Please enter a valid username!"),
  email: z.string().email("Please enter a valid email!"),
  password: z.string().min(6, "Please enter a valid password of at least 6 characters!"),
  picture: z.instanceof(File, { message: "Please provide a valid profile picture!" }),
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
    const validation = await registerSchema.spa(Object.fromEntries(formData.entries()));
    if (!validation.success) {
      return fail(400, { validationError: validation.error.flatten().fieldErrors });
    }
    const res = await fetchBackend("/auth/register", {
      authz: false,
      type: "multipart",
      cookies,
      fetch,
      setHeaders,
      method: "POST",
      body: formData,
      signal: AbortSignal.timeout(10000),
    });
    if (!res.ok) {
      return fail(res.status, { error: res.error });
    }
    redirect(307, `${base}/auth/login`);
  },
};
