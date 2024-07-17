<script lang="ts" generics="T extends keyof SvelteHTMLElements">
  import { clsx } from "$lib/clsx";
  import type { SvelteHTMLElements } from "svelte/elements";

  type CommentRendererButtonProps = Omit<SvelteHTMLElements[T], "class"> & {
    as: T;
    hoverable?: boolean;
  };

  const { as, hoverable = true, children, ...props }: CommentRendererButtonProps = $props();
</script>

<!-- We hard-code this element's height to 36px so that its radii is predictable (18px) -->
<svelte:element
  this={as}
  class={clsx(
    "flex h-8 select-none items-center justify-center gap-1 rounded-full border px-2 py-1 text-base shadow-md",
    "border-border-light dark:border-border-dark dark:bg-neutral-915 cursor-pointer bg-white",
    hoverable && "hover:bg-neutral-250 transition-colors duration-100 dark:hover:bg-neutral-800"
  )}
  {...props}
>
  {#if children}
    {@render children()}
  {/if}
</svelte:element>
