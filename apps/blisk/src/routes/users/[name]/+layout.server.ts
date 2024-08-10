import { fetchBackend } from "$lib/backend";
import { error } from "@sveltejs/kit";
import type { LayoutServerLoad } from "./$types";
import type { Post } from "$lib/types";

interface LoadData {
  name: string;
  posts: Post[];
}

export const load: LayoutServerLoad = async ({ cookies, fetch, params, setHeaders }) => {
  const user = await fetchBackend<LoadData>(`/users?username=${params.name}`, {
    authz: "optional",
    cookies,
    fetch,
    setHeaders,
  });
  if (!user.ok) {
    error(user.status, user.error);
  }
  return { title: `User ${user.data.name}`, data: user.data };
};
