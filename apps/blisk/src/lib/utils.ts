import { VALID_REACTIONS } from "./constants";

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

      if (isNaN(numQ)) {
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
