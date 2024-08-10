import { z } from "zod";
import { VALID_REACTION_FOR, VALID_REACTIONS } from "./constants";

const isValidLiteralUnion = <T extends z.ZodLiteral<unknown>>(literals: T[]): literals is [T, T, ...T[]] => literals.length >= 2;

const literalUnion = <T extends z.ZodLiteral<unknown>>(literals: T[], params?: z.RawCreateParams) => {
  if (!isValidLiteralUnion(literals)) {
    throw new Error("Literals passed do not meet the criteria for constructing a union schema, the minimum length is 2");
  }
  return z.union(literals, params);
};

export const errorSchema = z.object({
  error: z.string().min(1, "`error` is unexpectedly empty."),
});

export const successSchema = z.object({
  message: z.string().min(1, "`message` is unexpectedly empty."),
});

export const postIdSchema = z
  .number({ coerce: true, message: "Post ID is not a number!" })
  .int("Post ID must be an integer!")
  .safe("Post ID must be within safe range!");

export const commentIdSchema = z
  .number({ coerce: true, message: "Comment ID is not a number!" })
  .int("Comment ID must be an integer!")
  .safe("Comment ID must be within safe range!");

export const bookCategoryIdSchema = z
  .number({ coerce: true, message: "Category ID must be a number!" })
  .int("Category ID must be an integer!")
  .safe("Category ID must be within safe range!");

export const commentSchema = z.object({
  post_id: postIdSchema,
  parent_id: z
    .number({ coerce: true, message: "Parent ID is not a number!" })
    .int("Parent ID must be an integer!")
    .safe("Parent ID must be within safe range!")
    .nullable(),
  content: z.string().min(1, "Your comment must not be empty!"),
});

export const reactionTypeSchema = literalUnion(
  VALID_REACTIONS.map((reaction) => z.literal(reaction)),
  { message: "Reaction is not valid!" }
);

export const reactionForSchema = literalUnion(
  VALID_REACTION_FOR.map((reactionFor) => z.literal(reactionFor)),
  { message: "Reaction must be for a post or a comment!" }
);

export const reactionSchema = z.object({
  for_type: reactionForSchema,
  post_id: postIdSchema,
  reaction_type: reactionTypeSchema.or(z.literal("cancel")),
});

export const editCommentSchema = z.object({
  id: commentIdSchema,
  content: z.string().min(1, "Your comment must not be empty!"),
});
