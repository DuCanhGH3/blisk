<script lang="ts" generics="T extends keyof SvelteHTMLElements">
  import { clsx } from "$lib/clsx";
  import type { SvelteHTMLElements } from "svelte/elements";

  type CommentRendererButtonProps = Omit<SvelteHTMLElements[T], "class"> & {
    as: T;
    hoverable?: boolean;
    customColors?: string;
  };

  const {
    as,
    hoverable = true,
    customColors = clsx("dark:bg-wood-900 bg-wood-300", hoverable && "hover:bg-wood-200 dark:hover:bg-wood-800"),
    children,
    ...props
  }: CommentRendererButtonProps = $props();
</script>

<!-- We hard-code this element's height to 36px so that its radii is predictable (18px) -->
<svelte:element
  this={as}
  class={clsx(
    "flex h-8 select-none items-center justify-center gap-[5px] rounded-full border p-1 text-base shadow-md",
    "border-border-light dark:border-border-dark cursor-pointer transition-colors duration-100",
    customColors
  )}
  {...props}
>
  {#if children}
    {@render children()}
  {/if}
</svelte:element>
