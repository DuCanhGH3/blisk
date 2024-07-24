import dayjs from "dayjs";
import { z } from "zod";
import type { Actions, PageServerLoad } from "./$types";
import { fail } from "@sveltejs/kit";

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

export const actions: Actions = {
  async default({ request }) {
    const formData = await request.formData();

    const data = await readSchema.spa({
      startingDate: formData.get("startingDate"),
      endingDate: formData.get("endingDate"),
    });

    if (!data.success) {
      const flattenedErrors = data.error.flatten();
      return fail(400, { error: flattenedErrors.formErrors, validationError: flattenedErrors.fieldErrors });
    }
  },
};

export const load: PageServerLoad = () => {
  const now = dayjs();
  return {
    now: now.format("YYYY-MM-DD"),
    oneWeekLater: now.add(1, "week").format("YYYY-MM-DD"),
    oneMonthLater: now.add(1, "month").format("YYYY-MM-DD"),
    oneYearLater: now.add(1, "year").format("YYYY-MM-DD"),
  };
};
