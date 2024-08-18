import { base } from "$app/paths";
import { PUBLIC_BACKEND_URL } from "$env/static/public";
import { VALID_REACTIONS } from "./constants";
import type { Image } from "./types";

/**
 * Determines whether an array includes a certain element, returning true or false as appropriate.
 *
 * @param readonlyArray - The array of type `ReadonlyArray<T>`
 * @param searchElement - The element to search for.
 * @returns
 */
export const readonlyArrayIncludes = <T extends U, U>(readonlyArray: readonly T[], searchElement: U): searchElement is T =>
  readonlyArray.includes(searchElement as T);

export const isValidReaction = (reaction: unknown) => readonlyArrayIncludes(VALID_REACTIONS, reaction);

export interface ParseAcceptLanguageOptions {
  /**
   * The value from the `Accept-Language` header.
   * @see https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Accept-Language
   */
  acceptLanguage: string | null | undefined;
  /**
   * A validate callback that is called for each `locale`. If the `locale` is valid, return the `locale` as a string.
   * Otherwise return `undefined`, `null`, or an empty array.
   * Should the callback throw an error, the error will be caught and the `locale` will be ignored.
   */
  validate?(locale: string): string | string[] | null | undefined;
  /**
   * If set to `true`, the wildcard locale `*` will be returned in the array.
   * If set to `false`, the wildcard locale `*` will be ignored.
   *
   * @default true
   */
  ignoreWildcard?: boolean;
}

/**
 * Parse the `Accept-Language` header.
 *
 * Source: https://github.com/donavon/intl-parse-accept-language/blob/be8c28ba716b0fb48414d70ed7ff863c0876ef55/src/index.ts
 *
 * License: MIT
 *
 * @param options
 * @returns
 */
export const parseAcceptLanguage = ({
  acceptLanguage,
  validate = (locale: string) => locale,
  ignoreWildcard = true,
}: ParseAcceptLanguageOptions): string[] => {
  if (!acceptLanguage) {
    return [];
  }

  return acceptLanguage
    .split(",")
    .map((lang): [number, string] => {
      const [locale, q = "q=1"] = lang.split(";");

      const trimmedLocale = locale.trim();

      const numQ = Number(q.replace(/q ?=/, ""));

      if (Number.isNaN(numQ)) {
        return [0, trimmedLocale];
      }

      return [numQ, trimmedLocale];
    })
    .sort(([q1], [q2]) => q2 - q1)
    .flatMap(([_, locale]) => {
      if (locale === "*" && ignoreWildcard) {
        return [];
      }

      try {
        return validate(locale) || [];
      } catch {
        return [];
      }
    });
};

/**
 * Create a range used for iterating.
 *
 * Source: https://github.com/sodiray/radash/blob/069b26cdd7d62e6ac16a0ad3baa1c9abcca420bc/src/array.ts#L321-L334
 *
 * License: MIT
 *
 * @param startOrLength
 * @param end
 * @param mapper
 * @param step
 */
export function* range<T = number>(startOrLength: number, end?: number, mapper: (i: number) => T = (i) => i as T, step: number = 1): Generator<T> {
  const start = end ? startOrLength : 0;
  const final = end ?? startOrLength;
  for (let i = start; i <= final; i += step) {
    yield mapper(i);
    if (i + step > final) break;
  }
}

const isSafeRedirect = (to: unknown): to is `/${string}` => !!to && typeof to === "string" && to.startsWith("/") && !to.startsWith("//");

export const safeRedirect = (to: FormDataEntryValue | string | null | undefined, fallback = `${base}/`) => {
  if (!isSafeRedirect(to)) {
    to = fallback;
  }
  return to;
};

export const debounce = <Args extends any[], Ret>(fn: (...args: Args) => Ret, delay: number) => {
  let timer: NodeJS.Timeout | undefined = undefined;
  return (...args: Args) => {
    clearTimeout(timer);
    timer = setTimeout(() => {
      fn(...args);
    }, delay);
  };
};

export const getLoginUrl = (currentPath: string) =>
  isSafeRedirect(currentPath) ? `${base}/auth/login?redirectTo=${currentPath}` : `${base}/auth/login`;

export const getImage = (image: Image | null, fallback: string) =>
  image ? `${PUBLIC_BACKEND_URL}/assets/${image.owner}/${image.id}.${image.ext}` : fallback;

export const getProfilePicture = (profilePicture: Image | null) => getImage(profilePicture, "/no-avatar.webp");

// Source: https://stackoverflow.com/a/46774073
export const convertFormData = (formData: FormData) => {
  const object: Record<string, FormDataEntryValue | FormDataEntryValue[]> = {};
  formData.forEach((value, key) => {
    // If `object` does have `key`, this is the first entry.
    if (!Reflect.has(object, key)) {
      object[key] = value;
      return;
    }
    // If `object` does have `key`, it means that there are
    // duplicates of the same `key`. Therefore, we convert
    // `object[key]` to an array.
    if (!Array.isArray(object[key])) object[key] = [object[key]];
    object[key].push(value);
  });
  return object;
};
