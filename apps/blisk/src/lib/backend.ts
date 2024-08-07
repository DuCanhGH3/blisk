import { BACKEND_URL } from "$env/static/private";
import { fail, redirect, type Cookies } from "@sveltejs/kit";
import type { ReactionType, SetHeaders } from "./types";
import { editCommentSchema, errorSchema, reactionSchema } from "./schemas";

export type Authz = boolean | "optional";

export interface BackendInit extends RequestInit {
  authz: Authz;
  cookies: Cookies;
  fetch: typeof globalThis.fetch;
  setHeaders: SetHeaders;
}

export type BackendResult<T> = { ok: true; data: T } | { ok: false; status: number; error: string };

export const fetchBackend = async <T>(url: `/${string}`, { authz, cookies, fetch, setHeaders, ...init }: BackendInit): Promise<BackendResult<T>> => {
  const headers = new Headers({
    Accept: "application/json",
    "Content-Type": "application/json",
    ...init.headers,
  });
  if (authz) {
    const tokenType = cookies.get("token_type");
    const token = cookies.get("token");
    if (tokenType && token && tokenType === "Bearer") {
      headers.set("Authorization", `${tokenType} ${token}`);
    } else if (authz !== "optional") {
      setHeaders({
        "WWW-Authenticate":
          'Bearer realm="A protected resource requiring authorization", error="invalid_token", error_description="The access token is invalid."',
      });
      return { ok: false, status: 401, error: "You are unauthorized to make this request." };
    }
  }
  let res: Response;
  try {
    res = await fetch(`${BACKEND_URL}${url}`, {
      ...init,
      headers,
    });
  } catch (err) {
    console.log(err);
    if (err instanceof Error && err.name === "TimeoutError") {
      return { ok: false, status: 500, error: "Server is currently under heavy load. Sorry for the inconvenience." };
    }
    return { ok: false, status: 500, error: "Internal Server Error" };
  }
  if (res.status >= 300 && res.status <= 308 && res.headers.has("Location")) {
    redirect(res.status, new URL(res.headers.get("Location")!));
  }
  if (!res.ok) {
    let json: unknown;
    try {
      json = await res.json();
    } catch (err) {
      console.error(err);
      json = { error: "An unexpected error occurred." };
    }
    const validatedJson = await errorSchema.spa(json);
    if (!validatedJson.success) {
      return { ok: false, status: 500, error: "Internal Server Error" };
    }
    return { ok: false, status: res.status, error: validatedJson.data.error };
  }
  return { ok: true, data: await res.json() };
};

export const createReaction = async (formData: FormData, fetch: typeof globalThis.fetch, cookies: Cookies, setHeaders: SetHeaders) => {
  const data = await reactionSchema.spa({
    post_id: formData.get("forId"),
    for_type: formData.get("forType"),
    reaction_type: formData.get("reactionType"),
  });

  if (!data.success) {
    return fail(400, { validationError: data.error.flatten().fieldErrors });
  }

  const res = await fetchBackend<{ reaction_type: ReactionType }>("/reactions", {
    authz: true,
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

  return { reactionType: res.data.reaction_type };
};

export const editComment = async (formData: FormData, fetch: typeof globalThis.fetch, cookies: Cookies, setHeaders: SetHeaders) => {
  const data = await editCommentSchema.spa({
    id: formData.get("id"),
    content: formData.get("content"),
  });

  if (!data.success) {
    return fail(400, { validationError: data.error.flatten().fieldErrors });
  }

  const res = await fetchBackend("/comments", {
    authz: true,
    cookies,
    fetch,
    setHeaders,
    method: "PATCH",
    body: JSON.stringify(data.data),
    signal: AbortSignal.timeout(10000),
  });

  if (!res.ok) {
    return fail(res.status, { error: res.error });
  }
};
