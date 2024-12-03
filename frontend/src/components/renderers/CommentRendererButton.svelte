<script lang="ts" generics="T extends keyof SvelteHTMLElements">
  import { clsx } from "$lib/clsx";
  import type { SvelteHTMLElements } from "svelte/elements";

  type CommentRendererButtonProps = Omit<SvelteHTMLElements[T], "class"> & {
    as: T;
    interactive?: boolean;
    hoverable?: boolean;
    customColors?: string;
  };

  const {
    as,
    interactive = true,
    hoverable = true,
    customColors = clsx("dark:bg-wood-800 bg-wood-300", interactive && hoverable && "hover:bg-wood-200 dark:hover:bg-wood-750"),
    children,
    ...props
  }: CommentRendererButtonProps = $props();
</script>

<!-- We hard-code this element's height to 36px so that its radii is predictable (18px) -->
<svelte:element
  this={as}
  class={clsx(
    "flex h-8 select-none items-center justify-center gap-[5px] rounded-full border p-1 text-base shadow-md",
    "border-border-light dark:border-border-dark transition-colors duration-100",
    interactive && "cursor-pointer",
    customColors
  )}
  {...props}
>
  {#if children}
    {@render children()}
  {/if}
</svelte:element>
