import type { COLOR_SCHEMES } from "./constants";

export type ColorScheme = (typeof COLOR_SCHEMES)[number];

export interface User {
  name: string;
  email: string;
  role: string;
}

export interface Post {
  title: string;
  content: string;
  author_name: string;
}
