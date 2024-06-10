import { fail } from "@sveltejs/kit";
import { z } from "zod";

export const errorSchema = z.object({
  error: z.string().min(1, "`error` is unexpectedly empty."),
});
export const successSchema = z.object({
  message: z.string().min(1, "`message` is unexpectedly empty."),
});

export const handleBackendError = async (response: Response) => {
  let json: unknown;
  try {
    json = await response.json();
  } catch {
    json = { error: "An unexpected error occurred." };
  }
  const validatedJson = await errorSchema.spa(json);
  if (!validatedJson.success) {
    return fail(500, { error: "Internal Server Error" });
  }
  return fail(response.status, { error: validatedJson.data.error });
};

export const handleBackendSuccess = async (response: Response) => {
  return { message: await successSchema.parseAsync(await response.json()) };
};
