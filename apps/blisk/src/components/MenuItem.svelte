<script lang="ts" generics="T extends keyof SvelteHTMLElements">
  import { clsx } from "$lib/clsx";
  import type { SvelteHTMLElements } from "svelte/elements";

  type MenuItemProps = SvelteHTMLElements[T] & {
    as: T;
    customColors?: string;
  };

  const { as, customColors = "text-wood-900 dark:text-white", class: className, children, ...props }: MenuItemProps = $props();
</script>

<svelte:element
  this={as}
  class={clsx(
    "group flex w-full select-none items-center rounded-md p-2 text-sm transition-colors duration-100",
    "hover:bg-wood-200 dark:hover:bg-wood-750",
    customColors,
    className
  )}
  {...props}
>
  {#if children}
    {@render children()}
  {/if}
</svelte:element>
