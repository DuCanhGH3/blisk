import type { ReactionMetadata } from "./types";

export const COLOR_SCHEMES = ["dark", "light"] as const;

export const A_MONTH_IN_SECONDS = 60 * 60 * 24 * 30;

export const BREAKPOINTS = {
  md: 768,
  lg: 1280,
};

export const OPTIMISTIC_ID = -9999;

export const VALID_REACTIONS = ["like", "love", "laugh", "wow", "sad", "angry"] as const;

export const VALID_REACTION_FOR = ["post", "comment"] as const;

export const REPLY_DEPTH = 4;

export const BASE_COMMENTS_LENGTH = 20;

export const INITIAL_REACTION_METADATA = {
  total: 0,
  like: 0,
  love: 0,
  laugh: 0,
  wow: 0,
  sad: 0,
  angry: 0,
} satisfies ReactionMetadata;