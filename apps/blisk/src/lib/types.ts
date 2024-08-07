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

export type BookReaction = "like" | "dislike";

export interface Post {
  id: number;
  title: string;
  content: string;
  author_name: string;
  reaction: BookReaction;
  user_reaction: ReactionType | null;
}

export interface Comment {
  id: number;
  content: string;
  author_name: string;
  user_reaction: ReactionType | null;
  children: Comment[] | null;
}

export interface ClientComment extends Comment {
  isEditing: boolean;
  editText: string;
}

export interface BookAuthor {
  id: number;
  name: string;
}

export interface BookCategory {
  id: number;
  name: string;
}

export interface Book {
  title: string;
  summary: string;
  authors: BookAuthor[];
  categories: BookCategory[];
  reviews: Post[];
}

export type ReactionFor = z.infer<typeof reactionForSchema>;

export type ReactionType = z.infer<typeof reactionTypeSchema>;

export type SetHeaders = (headers: Record<string, string>) => void;

export type HeadingLevel = 1 | 2 | 3 | 4 | 5 | 6;

export type Ref<T> = { ref: T };