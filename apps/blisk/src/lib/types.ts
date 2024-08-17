import type { COLOR_SCHEMES, VALID_REACTION_FOR, VALID_REACTIONS } from "./constants";

export type RequireFields<T, K extends keyof T> = Omit<T, K> & {
  [P in keyof Pick<T, K>]-?: NonNullable<T[P]>;
};

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
  comments: Comment[] | null;
}

export interface Comment {
  id: number;
  post_id: number;
  content: string;
  author_name: string;
  user_reaction: ReactionType | null;
  children: Comment[] | null;
  // The following are states that will be attached
  // to the object as the user uses the app.
  edit_text?: string;
  is_editing?: boolean;
  is_processing_edit?: boolean | undefined;
  old_content?: string | undefined;
  error?: FormError<"content"> | undefined;
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
  name: string;
  summary: string;
  authors: BookAuthor[];
  categories: BookCategory[];
  reviews: Post[];
}

export type ReactionFor = (typeof VALID_REACTION_FOR)[number];

export type ReactionType = (typeof VALID_REACTIONS)[number];

export type SetHeaders = (headers: Record<string, string>) => void;

export type HeadingLevel = 1 | 2 | 3 | 4 | 5 | 6;

export type Ref<T> = { ref: T };

export type FormError<Fields extends string> = { error?: string | undefined; validationError: Record<Fields, string> };

export type ButtonType = "normal" | "light" | "error";

export type MouseEventEvent<T extends EventTarget> = MouseEvent & {
  currentTarget: EventTarget & T;
};
