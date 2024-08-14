import { PUBLIC_BACKEND_URL } from "$env/static/public";

export interface BackendInit extends RequestInit {
  noSuccessContent?: boolean;
}

export type BackendResult<T> = { ok: true; data: T } | { ok: false; status: number; error: string };

/**
 * Fetch data from the backend on the client. Unlike `backend.ts.fetchBackend`,
 * this does not use the `Authorization` header, validate data from the backend,
 * and handle redirects.
 * @param url
 * @param init
 * @returns
 */
export const fetchBackend = async <T>(
  url: `/${string}`,
  { noSuccessContent, credentials = "include", ...init }: BackendInit = {}
): Promise<BackendResult<T>> => {
  const headers = new Headers({
    Accept: "application/json",
    "Content-Type": "application/json",
    ...init.headers,
  });
  let res: Response;
  try {
    res = await fetch(`${PUBLIC_BACKEND_URL}${url}`, {
      ...init,
      credentials,
      headers,
    });
  } catch (err) {
    console.error("An error was thrown while fetching:", err);
    if (err instanceof Error && err.name === "TimeoutError") {
      return { ok: false, status: 500, error: "Server is currently under heavy load. Sorry for the inconvenience." };
    }
    return { ok: false, status: 500, error: "Internal Server Error" };
  }
  if (!res.ok) {
    let json: unknown;
    try {
      json = await res.json();
    } catch (err) {
      console.error("An error occurred while parsing error response:", err);
      json = { error: "An unexpected error occurred." };
    }
    if (json && typeof json === "object") {
      if ("validation_error" in json) {
        console.error("An unexpected validation error occurred:", json.validation_error);
        return { ok: false, status: 500, error: "Internal Server Error" };
      }
      if ("error" in json && typeof json.error === "string") {
        return { ok: false, status: res.status, error: json.error };
      }
    }
    return { ok: false, status: 500, error: "Internal Server Error" };
  }
  if (noSuccessContent) {
    return { ok: true, data: undefined as unknown as T };
  }
  return { ok: true, data: await res.json() };
};
