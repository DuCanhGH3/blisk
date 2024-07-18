import { z } from "zod";
import { VALID_REACTIONS } from "./constants";

const isValidLiteralUnion = <T extends z.ZodLiteral<unknown>>(literals: T[]): literals is [T, T, ...T[]] => literals.length >= 2;

const literalUnion = <T extends z.ZodLiteral<unknown>>(literals: T[], params?: z.RawCreateParams) => {
  if (!isValidLiteralUnion(literals)) {
    throw new Error("Literals passed do not meet the criteria for constructing a union schema, the minimum length is 2");
  }
  return z.union(literals, params);
};

export const reactionTypeSchema = literalUnion(
  VALID_REACTIONS.map((reaction) => z.literal(reaction)),
  { message: "Reaction is not valid!" }
);
