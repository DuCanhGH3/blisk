import type { NavHrefInfo } from "./navbar-types";

export const GENERAL_LINKS = [{ label: "Home", link: "/" }] satisfies NavHrefInfo[];

export const LOGGED_IN_LINKS = [] satisfies NavHrefInfo[];

export const LOGGED_OUT_LINKS = [{ label: "Login", link: "/login" }] satisfies NavHrefInfo[];
