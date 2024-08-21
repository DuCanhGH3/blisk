import dayjs from "dayjs";
import { z } from "zod";
import { error, fail } from "@sveltejs/kit";
import { convertFormData } from "$lib/utils";
import { fetchBackend } from "$lib/backend";
import type { Book } from "$lib/types";

const readSchema = z
  .object({
    startingDate: z.string().date("`startingDate` must be a valid date!"),
    endingDate: z.string().date("`endingDate` must be a valid date!"),
  })
  .superRefine((val, ctx) => {
    const startingDate = dayjs(val.startingDate);
    const endingDate = dayjs(val.endingDate);
    if (endingDate.isBefore(startingDate)) {
      ctx.addIssue({
        code: z.ZodIssueCode.custom,
        message: "You cannot begin reading after finishing...",
      });
    }

    return z.NEVER;
  });

export const actions = {
  async default({ request }) {
    const data = await readSchema.spa(convertFormData(await request.formData()));

    if (!data.success) {
      const flattenedErrors = data.error.flatten();
      return fail(400, { error: flattenedErrors.formErrors, validationError: flattenedErrors.fieldErrors });
    }
  },
};

export const load = async ({ cookies, fetch, setHeaders }) => {
  const books = await fetchBackend<Omit<Book, "reviews">[]>("/books?include_reviews=false", {
    authz: false,
    cookies,
    fetch,
    setHeaders,
  });
  if (!books.ok) {
    error(books.status, books.error);
  }
  const now = dayjs();
  return {
    title: "Read",
    books: books.data,
    now: now.format("YYYY-MM-DD"),
    oneWeekLater: now.add(1, "week").format("YYYY-MM-DD"),
    oneMonthLater: now.add(1, "month").format("YYYY-MM-DD"),
    oneYearLater: now.add(1, "year").format("YYYY-MM-DD"),
  };
};
