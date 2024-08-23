<script lang="ts" generics="T extends keyof SvelteHTMLElements">
  import type { SvelteHTMLElements } from "svelte/elements";
  import { clsx } from "$lib/clsx";
  import type { ButtonType } from "$lib/types";

  type ButtonProps = SvelteHTMLElements[T] & {
    as: T;
    variant?: ButtonType;
  };

  const { as, variant = "normal", class: className, children, ...props }: ButtonProps = $props();

  const colorClass = $derived.by(() => {
    switch (variant) {
      case "normal":
        return clsx(
          "bg-accent-light text-white dark:bg-accent-dark dark:text-wood-900",
          "[&:not(:disabled)]:hover:bg-accent-light/80 dark:[&:not(:disabled)]:hover:bg-accent-dark/80",
          "disabled:bg-gray-100 disabled:text-neutral-700 dark:disabled:bg-neutral-900 dark:disabled:text-neutral-400"
        );
      case "light":
        return clsx(
          "text-accent-light dark:text-accent-dark dark:bg-neutral-915 bg-white",
          "dark:[&:not(:disabled)]:hover:bg-neutral-800 [&:not(:disabled)]:hover:bg-neutral-150",
          "disabled:bg-gray-100 disabled:text-neutral-700 dark:disabled:bg-neutral-900 dark:disabled:text-neutral-400"
        );
      case "error":
        return clsx(
          "bg-error-light dark:bg-error-dark text-white dark:text-wood-900",
          "[&:not(:disabled)]:hover:bg-error-hover-light dark:[&:not(:disabled)]:hover:bg-error-hover-dark",
          "disabled:bg-error-disabled-light disabled:text-gray-100 dark:disabled:bg-error-disabled-dark dark:disabled:text-neutral-800"
        );
    }
  });
</script>

<svelte:element
  this={as}
  class={clsx(
    "transition-colors-opacity min-w-max rounded-md px-5 py-2.5 text-center text-sm font-medium duration-100",
    "focus:outline-none focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800",
    "border-border-light dark:border-border-dark border shadow-md  select-none flex flex-row gap-1",
    colorClass,
    className
  )}
  {...props}
>
  {#if children}
    {@render children()}
  {/if}
</svelte:element>
