import { BACKEND_URL } from "$env/static/private";
import { fail, redirect, type Cookies } from "@sveltejs/kit";
import type { ReactionType, SetHeaders } from "./types";
import { commentSchema, editCommentSchema, errorSchema, reactionSchema } from "./schemas";
import { convertFormData } from "./utils";

export type Authz = boolean | "optional";

export type RequestType = "json" | "multipart" | "url-encoded";

export interface BackendInit extends RequestInit {
  authz: Authz;
  type?: RequestType;
  cookies: Cookies;
  fetch: typeof globalThis.fetch;
  setHeaders: SetHeaders;
  noSuccessContent?: boolean;
}

export type BackendResult<T> = { ok: true; data: T } | { ok: false; status: number; error: string };

/**
 * Fetch data from the backend on the server. Useful for actions, `load`
 * functions, and more.
 * @param url
 * @param init
 * @returns
 */
export const fetchBackend = async <T>(
  url: URL | `/${string}`,
  { authz, type = "json", cookies, fetch, setHeaders, noSuccessContent, ...init }: BackendInit
): Promise<BackendResult<T>> => {
  const requestType = type === "json" ? "application/json" : type === "url-encoded" ? "application/x-www-form-urlencoded" : null;
  const headers = new Headers({
    Accept: "application/json",
    ...(requestType ? { "Content-Type": requestType } : {}),
    ...init.headers,
  });
  if (authz) {
    const token = cookies.get("token");
    if (token) {
      headers.set("Authorization", `Bearer ${token}`);
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
    res = await fetch(url instanceof URL ? url : `${BACKEND_URL}${url}`, {
      ...init,
      headers,
    });
  } catch (err) {
    console.error("An error was thrown while fetching:", err);
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
      console.error("An error occurred while parsing error response:", err);
      json = { error: "An unexpected error occurred." };
    }
    if (json && typeof json === "object" && "validation_error" in json) {
      console.error("An unexpected validation error occurred:", json.validation_error);
      return { ok: false, status: 500, error: "Internal Server Error" };
    }
    const validatedJson = await errorSchema.spa(json);
    if (!validatedJson.success) {
      return { ok: false, status: 500, error: "Internal Server Error" };
    }
    return { ok: false, status: res.status, error: validatedJson.data.error };
  }
  if (noSuccessContent) {
    return { ok: true, data: undefined as unknown as T };
  }
  return { ok: true, data: await res.json() };
};

export const createReaction = async (formData: FormData, fetch: typeof globalThis.fetch, cookies: Cookies, setHeaders: SetHeaders) => {
  const data = await reactionSchema.spa(convertFormData(formData));

  if (!data.success) {
    return fail(400, { validationError: data.error.flatten().fieldErrors });
  }

  if (data.data.reaction_type === "cancel") {
    const res = await fetchBackend("/reactions", {
      authz: true,
      cookies,
      fetch,
      setHeaders,
      noSuccessContent: true,
      method: "DELETE",
      body: JSON.stringify({
        for_type: data.data.for_type,
        post_id: data.data.post_id,
      }),
      signal: AbortSignal.timeout(10000),
    });

    if (!res.ok) {
      return fail(res.status, { error: res.error });
    }
  } else {
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
  }
};

export const createComment = async (
  postId: string,
  parentId: string | null,
  formData: FormData,
  fetch: typeof globalThis.fetch,
  cookies: Cookies,
  setHeaders: SetHeaders
) => {
  const data = await commentSchema.spa({
    post_id: postId,
    parent_id: parentId,
    content: formData.get("content"),
  });

  if (!data.success) {
    return fail(400, { validationError: data.error.flatten().fieldErrors });
  }

  const res = await fetchBackend<{ id: number }>("/comments", {
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

  return { id: res.data.id };
};

export const editComment = async (formData: FormData, fetch: typeof globalThis.fetch, cookies: Cookies, setHeaders: SetHeaders) => {
  const data = await editCommentSchema.spa(convertFormData(formData));

  if (!data.success) {
    return fail(400, { validationError: data.error.flatten().fieldErrors });
  }

  const res = await fetchBackend("/comments", {
    authz: true,
    cookies,
    fetch,
    setHeaders,
    noSuccessContent: true,
    method: "PATCH",
    body: JSON.stringify(data.data),
    signal: AbortSignal.timeout(10000),
  });

  if (!res.ok) {
    return fail(res.status, { error: res.error });
  }
};
