import type { z } from "zod";
import type { COLOR_SCHEMES } from "./constants";
import type { reactionForSchema, reactionTypeSchema } from "./schemas";

export type RequireFields<T, K extends keyof T> = Omit<T, K> & Required<Pick<T, K>>;

export type ColorScheme = (typeof COLOR_SCHEMES)[number];

export interface User {
  name: string;
  email: string;
  role: string;
}

export type BookReaction =
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
  id: number;
  title: string;
  content: string;
  author_name: string;
  reaction: BookReaction;
}

export interface Comment {
  id: number;
  path: string;
  content: string;
  author_name: string;
  user_reaction: ReactionType | null;
  level: number;
  post_id: number;
  replies: Comment[];
}

export interface Book {
  title: string;
  summary: string;
  reviews: Post[];
}

export type ReactionFor = z.infer<typeof reactionForSchema>;

export type ReactionType = z.infer<typeof reactionTypeSchema>;
