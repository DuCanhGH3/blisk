import type { NavHrefInfo } from "./navbar-types";

export const GENERAL_LINKS = [{ label: "home", link: "/" }] satisfies NavHrefInfo[];

export const LOGGED_IN_LINKS = [{ label: "post", link: "/posts" }] satisfies NavHrefInfo[];

export const LOGGED_OUT_LINKS = [{ label: "login", link: "/login" }] satisfies NavHrefInfo[];
