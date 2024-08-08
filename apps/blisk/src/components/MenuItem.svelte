<script lang="ts" generics="T extends keyof SvelteHTMLElements">
  import { clsx } from "$lib/clsx";
  import type { SvelteHTMLElements } from "svelte/elements";

  type MenuItemProps = SvelteHTMLElements[T] & {
    as: T;
    customColors?: string;
  };

  const { as, customColors = "text-black dark:text-white", class: className, children, ...props }: MenuItemProps = $props();
</script>

<svelte:element
  this={as}
  class={clsx(
    "group flex w-full select-none items-center rounded-md p-2 text-sm transition-colors duration-100",
    "hover:bg-gray-200  hover:dark:bg-neutral-800",
    customColors,
    className
  )}
  {...props}
>
  {#if children}
    {@render children()}
  {/if}
</svelte:element>
