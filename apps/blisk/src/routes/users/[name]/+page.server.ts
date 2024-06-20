import { fetchBackend } from "$lib/backend";
import { error } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ cookies, fetch, params, setHeaders }) => {
  const res = await fetchBackend(`/users/read?username=${params.name}`, {
    authz: false,
    cookies,
    fetch,
    setHeaders,
  });
  if (!res.ok) {
    error(res.status, res.error);
  }
  return { data: res.data };
};
