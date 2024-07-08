<script lang="ts" generics="T extends keyof SvelteHTMLElements">
  import type { SvelteHTMLElements } from "svelte/elements";

  type PostRendererButtonProps = Omit<SvelteHTMLElements[T], "class"> & {
    as: T;
  };

  const { as, children, ...props }: PostRendererButtonProps = $props();
</script>

<!-- We hard-code this element's height to 40px so that its radii is predictable (20px) -->
<svelte:element
  this={as}
  class="border-border-light dark:border-border-dark dark:bg-neutral-915 flex h-10 flex-1 select-none items-center justify-center gap-1 rounded-full border bg-white p-1 text-base shadow-md"
  {...props}
>
  {#if children}
    {@render children()}
  {/if}
</svelte:element>
