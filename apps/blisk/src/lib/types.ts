import type { COLOR_SCHEMES } from "./constants";

export type RequireFields<T, K extends keyof T> = Omit<T, K> & Required<Pick<T, K>>;

export type ColorScheme = (typeof COLOR_SCHEMES)[number];

export interface User {
  name: string;
  email: string;
  role: string;
}

export type PostReaction =
  | "overwhelmingly_positive"
  | "very_positive"
  | "positive"
  | "mostly_positive"
  | "mixed"
  | "mostly_negative"
  | "negative"
  | "very_negative"
  | "overwhelmingly_negative";

export interface Post {
  title: string;
  content: string;
  author_name: string;
  reaction: PostReaction;
}

export interface Comment {
  id: number;
  path: string;
  content: string;
  author_name: string;
  level: number;
  post_id: number;
  replies: Comment[];
}

export interface Book {
  title: string;
  summary: string;
  reviews: Post[];
}
