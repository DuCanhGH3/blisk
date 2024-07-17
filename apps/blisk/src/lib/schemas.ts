import { z } from "zod";

export const reactionTypeSchema = z.union(
  [z.literal("like"), z.literal("love"), z.literal("laugh"), z.literal("wow"), z.literal("sad"), z.literal("angry")],
  { message: "Reaction is not valid!" }
);
