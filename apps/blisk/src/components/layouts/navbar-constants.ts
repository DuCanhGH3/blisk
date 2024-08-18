import type { NavHrefInfo } from "./navbar-types";

export const GENERAL_LINKS = [
  { label: "home", link: "/" },
  { label: "books", link: "/books" },
] satisfies NavHrefInfo[];

export const LOGGED_IN_LINKS = [{ label: "posts", link: "/posts" }] satisfies NavHrefInfo[];
