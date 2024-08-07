<script lang="ts" generics="T extends keyof SvelteHTMLElements">
  import { clsx } from "$lib/clsx";
  import type { SvelteHTMLElements } from "svelte/elements";

  type MenuItemProps = SvelteHTMLElements[T] & {
    as: T;
  };

  const { as, class: className, children, ...props }: MenuItemProps = $props();
</script>

<svelte:element
  this={as}
  class={clsx(
    "group flex w-full items-center rounded-md p-2 text-sm transition-colors duration-100 select-none",
    "text-black hover:bg-gray-200 dark:text-white hover:dark:bg-neutral-800",
    className
  )}
  {...props}
>
  {#if children}
    {@render children()}
  {/if}
</svelte:element>
