<script lang="ts" generics="T extends keyof SvelteHTMLElements">
  import { clsx } from "$lib/clsx";
  import type { SvelteHTMLElements } from "svelte/elements";

  type PostRendererButtonProps = Omit<SvelteHTMLElements[T], "class"> & {
    as: T;
    hoverable?: boolean;
    customColors?: string;
  };

  const {
    as,
    hoverable = true,
    customColors = clsx("dark:bg-neutral-915 bg-white", hoverable && "hover:bg-neutral-250 dark:hover:bg-neutral-800"),
    children,
    ...props
  }: PostRendererButtonProps = $props();
</script>

<!-- We hard-code this element's height to 40px so that its radii is predictable (20px) -->
<svelte:element
  this={as}
  class={clsx(
    "select-none items-center justify-center gap-1 rounded-full p-1 text-base shadow-md",
    "border-border-light dark:border-border-dark flex h-10 flex-1 border transition-colors duration-100",
    customColors
  )}
  {...props}
>
  {#if children}
    {@render children()}
  {/if}
</svelte:element>
