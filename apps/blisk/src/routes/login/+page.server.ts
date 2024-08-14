import { fail, redirect } from "@sveltejs/kit";
import { z } from "zod";
import { base } from "$app/paths";
import { fetchBackend } from "$lib/backend";
import { BACKEND_URL } from "$env/static/private";
import { AUTHORIZATION_REQUEST_PARAMS } from "$lib/constants";
import type { Actions, PageServerLoad } from "./$types";
// import type { AuthzRequestParams } from "$lib/types";

const loginSchema = z.object({
  username: z.string().min(1, "Please enter a valid username!"),
  password: z.string().min(1, "Please enter a valid password!"),
});

// TODO: determine if we should reauthn user via `prompt`.
// TODO: support `ui_locales` and `claims_locales`.
export const load: PageServerLoad = ({ locals }) => {
  if (locals.user) {
    redirect(303, `${base}/`);
  }
  // const authzParams = AUTHORIZATION_REQUEST_PARAMS.reduce(
  //   (prev, cur) => {
  //     const param = url.searchParams.get(cur);
  //     prev[cur] = param;
  //     return prev;
  //   },
  //   {} as Record<AuthzRequestParams, string | null>
  // );
  // const fetchUrl = new URL(`${BACKEND_URL}/auth/integration/validate-login`);
  // for (const params of AUTHORIZATION_REQUEST_PARAMS) {
  //   const searchParams = url.searchParams.get(params);
  //   if (searchParams) {
  //     fetchUrl.searchParams.set(params, searchParams);
  //   }
  // }
  return {
    title: "Login",
  };
};

export const actions: Actions = {
  async default({ cookies, fetch, request, setHeaders, url }) {
    try {
      const formData = await request.formData();
      const data = await loginSchema.spa({
        username: formData.get("username"),
        password: formData.get("password"),
      });
      if (!data.success) {
        return fail(400, { validationError: data.error.flatten().fieldErrors });
      }
      const fetchUrl = new URL(`${BACKEND_URL}/auth/integration/login`);
      for (const params of AUTHORIZATION_REQUEST_PARAMS) {
        const searchParams = url.searchParams.get(params);
        if (searchParams) {
          fetchUrl.searchParams.set(params, searchParams);
        }
      }
      const res = await fetchBackend<{ token_type: string; expires_in: number; id_token: string }>(fetchUrl, {
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
    redirect(303, `${base}/`);
  },
  // async logout({ cookies, locals }) {
  //   try {
  //     const cookiesOptions = {
  //       httpOnly: true,
  //       secure: true,
  //       sameSite: "strict",
  //       path: "/",
  //     } as const;
  //     cookies.delete("token_type", cookiesOptions);
  //     cookies.delete("token", cookiesOptions);
  //     locals.user = null;
  //   } catch (err) {
  //     console.error(err);
  //     return fail(500, { error: "Internal Server Error" });
  //   }
  // },
};
