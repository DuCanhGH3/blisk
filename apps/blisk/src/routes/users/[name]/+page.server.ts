import { fetchBackend } from "$lib/backend";
import { error } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";
import type { Post } from "$lib/types";

interface LoadData {
  name: string;
  posts: Post[];
}

export const load: PageServerLoad = async ({ cookies, fetch, params, setHeaders, url }) => {
  const user = await fetchBackend<LoadData>(`/users?username=${params.name}`, {
    authz: false,
    cookies,
    fetch,
    setHeaders,
  });
  if (!user.ok) {
    error(user.status, user.error);
  }
  return { title: `User ${user.data.name}`, data: user.data };
};
