import type { NavHrefInfo } from "./navbar-types";

export const GENERAL_LINKS = [
  { label: "Home", link: "/" },
  { label: "Books", link: "/books" },
  { label: "Posts", link: "/posts" },
] satisfies NavHrefInfo[];

export const LOGGED_IN_LINKS = [] satisfies NavHrefInfo[];
