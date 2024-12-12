import dayjs from "dayjs";
import { z } from "zod";
import { error, fail, redirect } from "@sveltejs/kit";
import { convertFormData } from "$lib/utils";
import { fetchBackend } from "$lib/backend";
import type { Book } from "$lib/types";
import { base } from "$app/paths";

const readSchema = z
  .object({
    // Since we only provide options rather than text inputs, error messages
    // are a bit less user-friendly.
    book_name: z.string().min(1, "`name` must point to a valid book!"),
    starts_at: z.string().date("`startingDate` must be a valid date!"),
    ends_at: z.string().date("`endingDate` must be a valid date!"),
  })
  .superRefine((val, ctx) => {
    const startingDate = dayjs(val.starts_at);
    const endingDate = dayjs(val.ends_at);
    if (endingDate.isBefore(startingDate)) {
      ctx.addIssue({
        code: z.ZodIssueCode.custom,
        message: "You cannot begin reading after finishing...",
      });
    }
    if (endingDate.diff(startingDate, "day") > 365) {
      ctx.addIssue({
        code: z.ZodIssueCode.custom,
        message: "Isn't that too much time to read a book?",
      });
    }
    return z.NEVER;
  });

export const actions = {
  async default(event) {
    const data = await readSchema.spa(convertFormData(await event.request.formData()));

    if (!data.success) {
      const flattenedErrors = data.error.flatten();
      return fail(400, { error: flattenedErrors.formErrors, validationError: flattenedErrors.fieldErrors });
    }

    const res = await fetchBackend("/books/read", {
      authz: true,
      type: "url-encoded",
      event,
      noSuccessContent: true,
      method: "POST",
      body: new URLSearchParams(data.data),
    });

    if (!res.ok) {
      return fail(res.status, { error: res.error });
    }

    redirect(307, `${base}/books/tracking/${data.data.book_name}`);
  },
};

export const load = async (event) => {
  const books = await fetchBackend<Omit<Book, "reviews">[]>("/books?include_reviews=false", {
    authz: false,
    event,
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
